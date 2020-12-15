package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.Menu
import android.view.MenuItem
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivitySettingsBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.inputString

class SettingsActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivitySettingsBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivitySettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)
        setSupportActionBar(binding.toolbar.root)

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        binding.instance.editText?.setText(common.instance)
    }

    fun saveClick(view: View?) {
        common.hideKeyboard()
        val editor = preferences.edit()
        val instance = inputString(binding.instance)
        if (instance.isEmpty()) {
            editor.remove("instance")
        } else {
            editor.putString("instance", common.encrypt(common.pinHash, instance))
        }
        editor.apply()
        common.decryptLogin(preferences)
        common.toast(R.string.settings_saved)
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}
