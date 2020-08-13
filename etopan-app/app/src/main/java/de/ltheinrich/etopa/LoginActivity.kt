package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.View
import android.view.inputmethod.EditorInfo
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.utils.Common

class LoginActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var instance: TextView
    private lateinit var username: TextView
    private lateinit var password: TextView
    private lateinit var key: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_login)
        System.loadLibrary("etopan")

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        instance = findViewById(R.id.instance);
        username = findViewById(R.id.username);
        password = findViewById(R.id.password);
        key = findViewById(R.id.key);

        key.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO) {
                loginClick(null)
            }
            false
        }
    }

    fun loginClick(view: View?) {
        common.hideKeyboard()
        (view ?: key).visibility = View.INVISIBLE
        if (instance.text.isNotEmpty() && username.text.isNotEmpty() && password.text.isNotEmpty() && key.text.isNotEmpty()) {
            val editor = preferences.edit()
            common.instance = instance.text.toString()
            common.username = username.text.toString()
            common.passwordHash = common.hashPassword(password.text.toString())
            common.keyHash = common.hashKey(key.text.toString())
            common.request("user/login", { response ->
                if (!response.has("token")) {
                    Toast.makeText(this, response.getString("error"), Toast.LENGTH_LONG).show()
                    //common.toast(R.string.incorrect_login)
                } else {
                    val token = response.getString("token")
                    Toast.makeText(this, token, Toast.LENGTH_LONG).show()
                }
            }, Pair("username", common.username), Pair("password", common.passwordHash))
            /*
            editor.putString("instance", common.encrypt(common.pinHash, common.instance))
            editor.putString("username", common.encrypt(common.pinHash, common.username))
            editor.putString("passwordHash", common.encrypt(common.pinHash, common.passwordHash))
            editor.putString("keyHash", common.encrypt(common.pinHash, common.keyHash))
            editor.apply()
            common.openActivity(AppActivity::class)*/
        } else {
            common.toast(R.string.inputs_empty)
        }
        (view ?: key).visibility = View.VISIBLE
    }
}
