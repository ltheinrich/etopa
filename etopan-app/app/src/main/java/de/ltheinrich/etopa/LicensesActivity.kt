package de.ltheinrich.etopa

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityLicensesBinding

class LicensesActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityLicensesBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityLicensesBinding.inflate(layoutInflater)
        setContentView(binding.root)
        binding.licenses.text = assets.open("NOTICE.txt").bufferedReader().use { it.readText() }
    }
}