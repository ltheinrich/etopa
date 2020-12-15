package de.ltheinrich.etopa

import android.content.SharedPreferences
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityLicensesBinding
import de.ltheinrich.etopa.utils.Common

class LicensesActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityLicensesBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityLicensesBinding.inflate(layoutInflater)
        setContentView(binding.root)
        binding.licenses.text = assets.open("NOTICE.txt").bufferedReader().use { it.readText() }
    }
}