package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.KeyEvent
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import de.ltheinrich.etopa.databinding.ActivityAppBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.Storage
import de.ltheinrich.etopa.utils.TokenAdapter

class AppActivity : AppCompatActivity() {

    val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private val tokens = ArrayList<Pair<String, String>>()
    private val handler = Handler(Looper.getMainLooper())
    private lateinit var storage: Storage
    private lateinit var binding: ActivityAppBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityAppBinding.inflate(layoutInflater)
        setContentView(binding.root)

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        binding.rvTokens.adapter = TokenAdapter(tokens, this)
        binding.rvTokens.layoutManager = LinearLayoutManager(this)

        common.requestString("data/get_secure",
            { secureStorage ->
                handleStorage(secureStorage)
            },
            Pair("username", common.username),
            Pair("token", common.token),
            error_handler = {
                common.toast(R.string.network_unreachable)
                preferences.getString("secretStorage", null)?.let {
                    val secureStorage =
                        common.decrypt(common.pinHash, it)
                    handleStorage(secureStorage, false)
                }
            })
    }

    private fun handleStorage(secureStorage: String, update: Boolean = true) {
        storage = Storage(common, secureStorage)
        if (storage.map.containsValue(""))
            common.toast(R.string.decryption_failed)
        else if (update)
            preferences.edit()
                .putString("secretStorage", common.encrypt(common.pinHash, secureStorage)).apply()
        handleTokens()
    }

    private fun handleTokens() {
        updateTokens()
        object : Runnable {
            override fun run() = try {
                val timeLeft = (System.currentTimeMillis() / 1000 % 30).toDouble()
                if (timeLeft < 1)
                    updateTokens()
                binding.time.progress = 100 - (timeLeft / 30 * 100).toInt()
            } finally {
                handler.postDelayed(this, 1000)
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
        binding.rvTokens.adapter?.notifyDataSetChanged()
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            common.openActivity(LoginActivity::class, Pair("noAutoLogin", "false"))
            return true
        }

        return super.onKeyDown(keyCode, event)
    }
}