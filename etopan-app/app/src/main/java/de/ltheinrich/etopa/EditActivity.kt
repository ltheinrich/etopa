package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.text.Editable
import android.text.TextWatcher
import android.util.Log
import android.view.Menu
import android.view.MenuItem
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityEditBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.inputString


class EditActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityEditBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityEditBinding.inflate(layoutInflater)
        setContentView(binding.root)
        val secretName = intent.getStringExtra("secretName").orEmpty()
        binding.toolbar.root.title =
            getString(R.string.app_name) + ": " + getString(R.string.edit_var, secretName)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)

        binding.deleteSecretCheck.setOnCheckedChangeListener { _, checked ->
            if (checked) {
                binding.deleteSecretName.visibility = View.VISIBLE
            } else {
                binding.deleteSecretName.visibility = View.GONE
                binding.deleteSecretName.editText?.text?.clear()
                binding.deleteSecretConfirm.isChecked = false
            }

        }

        binding.deleteSecretName.editText?.addTextChangedListener(object : TextWatcher {
            override fun beforeTextChanged(p0: CharSequence?, p1: Int, p2: Int, p3: Int) {}
            override fun onTextChanged(p0: CharSequence?, p1: Int, p2: Int, p3: Int) {}
            override fun afterTextChanged(name: Editable) {
                if (name.toString() == secretName) {
                    binding.deleteSecretConfirm.visibility = View.VISIBLE
                } else {
                    binding.deleteSecretConfirm.visibility = View.GONE
                    binding.deleteSecretConfirm.isChecked = false
                }
            }
        })

        binding.deleteSecretConfirm.setOnCheckedChangeListener { _, checked ->
            if (checked) {
                binding.deleteSecret.visibility = View.VISIBLE
            } else {
                binding.deleteSecret.visibility = View.GONE
            }
        }

        binding.deleteSecret.setOnClickListener {
            common.toast(R.string.sending_request)
            common.request(
                "data/delete",
                {
                    val error = it.getString("error")
                    if (error == "false") {
                        common.toast(R.string.secret_deleted)
                        common.openActivity(AppActivity::class)
                    } else {
                        common.toast(R.string.failed_error)
                        Log.d("API error", error)
                    }
                },
                Pair("secretname", common.hashName(secretName)),
                Pair("username", common.username),
                Pair("token", common.token)
            )
        }

        binding.renameSecret.setOnClickListener {
            val secretNewName = inputString(binding.secretNewName)
            if (secretNewName.isEmpty()) {
                common.toast(R.string.inputs_empty)
                return@setOnClickListener
            } else if (common.storage.map.containsKey(secretNewName)) {
                common.hideKeyboard(this)
                common.toast(R.string.name_exists)
                return@setOnClickListener
            }

            common.toast(R.string.sending_request)
            common.request(
                "data/rename",
                {
                    val error = it.getString("error")
                    if (error == "false") {
                        common.toast(R.string.success)
                        common.openActivity(AppActivity::class)
                    } else {
                        common.toast(R.string.failed_error)
                        Log.d("API error", error)
                    }
                },
                Pair("secretname", common.hashName(secretName)),
                Pair("newsecretname", common.hashName(secretNewName)),
                Pair("secretnameencrypted", common.encrypt(common.keyHash, secretNewName)),
                Pair("username", common.username),
                Pair("token", common.token)
            )
        }
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}
