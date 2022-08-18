package de.ltheinrich.etopa

import android.os.Bundle
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityLicensesBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.MenuType

class LicensesActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var binding: ActivityLicensesBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityLicensesBinding.inflate(layoutInflater)
        setContentView(binding.root)
        common.menuType = MenuType.DISABLED
        binding.licenses.text = assets.open("NOTICE.txt").bufferedReader().use { it.readText() }
        common.backActivity = AppActivity::class.java
        common.lockListener(this)

        binding.toolbar.root.title =
            getString(R.string.app_name) + ": " + getString(R.string.licenses)
        setSupportActionBar(binding.toolbar.root)

        supportActionBar?.setDisplayHomeAsUpEnabled(true)
        supportActionBar?.setDisplayShowHomeEnabled(true)
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?) = common.backKey(keyCode)
    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu): Boolean = common.createMenu(menu)
}