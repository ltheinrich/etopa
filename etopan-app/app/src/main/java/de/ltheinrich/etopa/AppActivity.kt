package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.*
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.Lifecycle
import androidx.recyclerview.widget.LinearLayoutManager
import de.ltheinrich.etopa.databinding.ActivityAppBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.Storage
import de.ltheinrich.etopa.utils.TokenAdapter

class AppActivity : AppCompatActivity() {

    val common: Common = Common.getInstance(this)
    private val tokens = ArrayList<Pair<String, String>>()
    private val handler = Handler(Looper.getMainLooper())
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityAppBinding
    private lateinit var selectedSecretName: String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityAppBinding.inflate(layoutInflater)
        setContentView(binding.root)
        common.extendedMenu = true
        binding.toolbar.root.title = getString(R.string.app_name)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        common.backActivity = MainActivity::class.java

        supportActionBar?.setDisplayHomeAsUpEnabled(true)
        supportActionBar?.setDisplayShowHomeEnabled(true)

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
        common.storage = Storage(common, secureStorage)
        if (common.storage.map.containsValue(""))
            common.toast(R.string.decryption_failed)
        else if (update)
            preferences.edit()
                .putString("secretStorage", common.encrypt(common.pinHash, secureStorage)).apply()
        handleTokens()
    }

    private fun handleTokens() {
        updateTokens()
        object : Runnable {
            override fun run() {
                try {
                    val timeLeft = (System.currentTimeMillis() / 1000 % 30).toDouble()
                    if (timeLeft < 1)
                        updateTokens()
                    if (lifecycle.currentState.isAtLeast(Lifecycle.State.RESUMED))
                        binding.time.progress = 100 - (timeLeft / 30 * 100).toInt()
                } finally {
                    handler.postDelayed(this, 1000)
                }
            }
        }.run()
    }

    private fun updateTokens() {
        tokens.clear()
        for (secret in common.storage.map) {
            tokens.add(
                Pair(
                    secret.key,
                    common.generateToken(secret.value)
                )
            )
        }
        binding.rvTokens.adapter?.notifyDataSetChanged()
    }

    override fun onCreateContextMenu(
        menu: ContextMenu?,
        v: View?,
        menuInfo: ContextMenu.ContextMenuInfo?,
    ) {
        super.onCreateContextMenu(menu, v, menuInfo)
        selectedSecretName = ((v as ViewGroup).getChildAt(0) as TextView).text.toString()
        v.id.let { menu?.add(it, Menu.FIRST, Menu.NONE, R.string.edit_secret) }
    }

    override fun onContextItemSelected(item: MenuItem) = when (item.itemId) {
        Menu.FIRST -> {
            common.openActivity(EditActivity::class, Pair("secretName", selectedSecretName))
            true
        }
        else -> {
            false
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?) = common.backKey(keyCode)
    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}
