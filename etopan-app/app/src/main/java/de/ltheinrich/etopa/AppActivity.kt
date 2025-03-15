package de.ltheinrich.etopa

import android.annotation.SuppressLint
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.view.ContextMenu
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.edit
import androidx.lifecycle.Lifecycle
import androidx.recyclerview.widget.LinearLayoutManager
import de.ltheinrich.etopa.databinding.ActivityAppBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.MenuType
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
        common.fixEdgeToEdge(findViewById(R.id.toolbar), findViewById(R.id.top_tokens_layout))
        common.menuType = MenuType.FULL
        binding.toolbar.root.title = getString(R.string.app_name)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        common.backActivity = MainActivity::class.java
        common.lockListener(this)

        supportActionBar?.setDisplayHomeAsUpEnabled(true)
        supportActionBar?.setDisplayShowHomeEnabled(true)

        binding.rvTokens.adapter = TokenAdapter(tokens, this)
        binding.rvTokens.layoutManager = LinearLayoutManager(this)

        common.requestString(
            "data/get_secure",
            { secureStorage ->
                handleStorage(secureStorage)
            },
            Pair("username", common.username),
            Pair("token", common.token),
            errorHandler = {
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
        if (common.storage!!.map.containsValue(""))
            common.toast(R.string.decryption_failed)
        else if (update)
            preferences.edit {
                putString("secretStorage", common.encrypt(common.pinHash, secureStorage))
            }
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

    @SuppressLint("NotifyDataSetChanged")
    private fun updateTokens() {
        tokens.clear()
        for (secret in common.storage!!.map) {
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
        if (common.offline) {
            menu?.close()
            common.toast(R.string.network_unreachable)
        } else {
            selectedSecretName = ((v as ViewGroup).getChildAt(0) as TextView).text.toString()
            v.id.let {
                menu?.add(it, Menu.FIRST, Menu.NONE, R.string.edit_secret)
                if (common.storage != null) {
                    if (!common.storage!!.isFirstSorted(selectedSecretName))
                        menu?.add(it, Menu.FIRST + 1, Menu.NONE, R.string.move_up)
                    if (!common.storage!!.isLastSorted(selectedSecretName))
                        menu?.add(it, Menu.FIRST + 2, Menu.NONE, R.string.move_down)
                }
            }
        }
    }

    override fun onContextItemSelected(item: MenuItem) = when (item.itemId) {
        Menu.FIRST -> {
            common.openActivity(EditActivity::class, Pair("secretName", selectedSecretName))
            true
        }

        Menu.FIRST + 1 -> {
            if (common.storage == null) {
                common.toast(R.string.unknown_error)
            } else {
                common.storage!!.moveUp(selectedSecretName)
                updateTokens()
                common.request(
                    "data/update_sort",
                    {
                        val error = it.getString("error")
                        if (error != "false") {
                            Log.e("API error", error)
                            common.toast(R.string.unknown_error)
                        }
                    },
                    Pair("username", common.username),
                    Pair("token", common.token),
                    Pair("secretssort", common.storage!!.encryptSort()),
                    errorHandler = {
                        common.toast(R.string.network_unreachable)
                    }
                )
            }
            true
        }

        Menu.FIRST + 2 -> {
            if (common.storage == null) {
                common.toast(R.string.unknown_error)
            } else {
                common.storage!!.moveDown(selectedSecretName)
                updateTokens()
                common.request(
                    "data/update_sort",
                    {
                        val error = it.getString("error")
                        if (error != "false") {
                            Log.e("API error", error)
                            common.toast(R.string.unknown_error)
                        }
                    },
                    Pair("username", common.username),
                    Pair("token", common.token),
                    Pair("secretssort", common.storage!!.encryptSort())
                )
            }
            true
        }

        else -> {
            false
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?) = common.backKey(keyCode)
    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu): Boolean = common.createMenu(menu)
}
