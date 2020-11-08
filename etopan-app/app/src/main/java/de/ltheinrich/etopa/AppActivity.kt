package de.ltheinrich.etopa

import android.content.SharedPreferences
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.KeyEvent
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.Storage
import de.ltheinrich.etopa.utils.TokenAdapter
import kotlinx.android.synthetic.main.activity_app.*

class AppActivity : AppCompatActivity() {

    val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private val tokens = ArrayList<Pair<String, String>>()
    private val handler = Handler(Looper.getMainLooper())
    private lateinit var storage: Storage

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app)

        rv_tokens.adapter = TokenAdapter(tokens, this)
        rv_tokens.layoutManager =
            LinearLayoutManager(this) //rv_secrets.layoutManager = GridLayoutManager(this, 2)

        common.requestString("data/get_secure",
            { response ->
                storage = Storage(common, response)
                if (storage.map.containsValue(""))
                    common.toast(R.string.decryption_failed)
                handleTokens()
            },
            Pair("username", common.username),
            Pair("token", common.token),
            error_handler = {
                common.toast(R.string.network_unreachable)
                if (preferences.contains("secretStorage")) {
                    common.openActivity(AppActivity::class)
                }
            })
    }

    private fun handleTokens() {
        updateTokens()
        object : Runnable {
            override fun run() {
                try {
                    if (System.currentTimeMillis() / 1000 % 30 == 0L)
                        updateTokens()
                } finally {
                    handler.postDelayed(this, 1000)
                }
            }
        }.run()
    }

    private fun updateTokens() {
        tokens.clear()
        for (secret in storage.map) {
            tokens.add(
                Pair(
                    secret.key,
                    common.generateToken(secret.value)
                )
            )
        }
        rv_tokens.adapter?.notifyDataSetChanged()
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            common.openActivity(LoginActivity::class, Pair("noAutoLogin", "false"))
            return true
        }

        return super.onKeyDown(keyCode, event)
    }
}