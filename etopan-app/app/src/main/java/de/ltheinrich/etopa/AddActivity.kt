package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityAddBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.inputString


class AddActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityAddBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityAddBinding.inflate(layoutInflater)
        setContentView(binding.root)
        binding.toolbar.root.title = getString(R.string.app_name) + ": " + getString(R.string.add)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)

        binding.addSecret.setOnClickListener {
            val secretName = inputString(binding.secretName)
            val secretValue = inputString(binding.secretValue)
            if (secretName.isEmpty() || secretValue.isEmpty()) {
                common.toast(R.string.inputs_empty)
                return@setOnClickListener
            } else if (common.storage.map.containsKey(secretName)) {
                common.hideKeyboard(this)
                common.toast(R.string.name_exists)
                return@setOnClickListener
            }

            common.toast(R.string.sending_request)
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
                error_handler = { common.toast(R.string.network_unreachable) }
            )
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            common.openActivity(AppActivity::class)
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}
