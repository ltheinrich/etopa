package de.ltheinrich.etopa

import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import android.view.inputmethod.EditorInfo
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.google.zxing.integration.android.IntentIntegrator
import de.ltheinrich.etopa.databinding.ActivityAddBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.MenuType
import de.ltheinrich.etopa.utils.inputString

class AddActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityAddBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityAddBinding.inflate(layoutInflater)
        setContentView(binding.root)
        common.menuType = MenuType.DISABLED
        binding.toolbar.root.title = getString(R.string.app_name) + ": " + getString(R.string.add)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        common.backActivity = AppActivity::class.java

        supportActionBar?.setDisplayHomeAsUpEnabled(true)
        supportActionBar?.setDisplayShowHomeEnabled(true)

        binding.secretValue.editText?.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO)
                addSecret()
            true
        }

        binding.addSecret.setOnClickListener {
            addSecret()
        }

        binding.qrCode.setOnClickListener {
            scanQRCode()
        }
    }

    private fun scanQRCode() {
        if (common.checkSdk(Build.VERSION_CODES.M) && checkSelfPermission(Manifest.permission.CAMERA) == PackageManager.PERMISSION_DENIED)
            requestPermissions(arrayOf(Manifest.permission.CAMERA), 1540)
        else {
            val integrator = IntentIntegrator(this).apply {
                setOrientationLocked(false)
                setDesiredBarcodeFormats(IntentIntegrator.ALL_CODE_TYPES)
                setBeepEnabled(false)
            }
            integrator.initiateScan()
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String?>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == 1540)
            if (grantResults[0] == PackageManager.PERMISSION_GRANTED)
                scanQRCode()
            else
                common.toast(R.string.permission_denied)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        val result = IntentIntegrator.parseActivityResult(requestCode, resultCode, data)
        if (result != null && result.contents != null) {
            val secret = common.parseSecretUri(result.contents)
            binding.secretValue.editText?.setText(secret.first)
            binding.secretName.editText?.setText(secret.second)
        } else {
            super.onActivityResult(requestCode, resultCode, data)
        }
    }

    private fun addSecret() {
        val secretName = inputString(binding.secretName)
        val secretValue = inputString(binding.secretValue).replace(" ", "")
        if (secretName.isEmpty() || secretValue.isEmpty()) {
            return common.toast(R.string.inputs_empty)
        }

        common.hideKeyboard(currentFocus)
        if (common.storage.map.containsKey(secretName)) {
            return common.toast(R.string.name_exists)
        } else if (common.generateToken(secretValue) == "invalid") {
            return common.toast(R.string.invalid_secret)
        }

        common.toast(R.string.sending_request, length = Toast.LENGTH_SHORT)
        common.request(
            "data/update",
            {
                val error = it.getString("error")
                if (error == "false") {
                    common.toast(R.string.secret_added)
                    common.openActivity(AppActivity::class)
                } else {
                    common.toast(R.string.failed_error)
                    Log.d("API error", error)
                }
            },
            Pair("secretname", common.hashName(secretName)),
            Pair("secretvalue", common.encrypt(common.keyHash, secretValue)),
            Pair("secretnameencrypted", common.encrypt(common.keyHash, secretName)),
            Pair("username", common.username),
            Pair("token", common.token),
            error_handler = {
                common.toast(R.string.network_unreachable)
            }
        )
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?) = common.backKey(keyCode)
    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
    override fun onPause() {
        common.lockOnPause()
        super.onPause()
    }
}
