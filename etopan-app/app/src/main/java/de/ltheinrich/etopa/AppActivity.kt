package de.ltheinrich.etopa

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import de.ltheinrich.etopa.utils.SecretAdapter
import kotlinx.android.synthetic.main.activity_app.*

class AppActivity() : AppCompatActivity() {

    val secrets = ArrayList<Pair<String, String>>()

    /*constructor(parcel: Parcel) : this() {
    }*/

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app)

        addSecrets()
        rv_secrets.layoutManager = LinearLayoutManager(this)
        //rv_secrets.layoutManager = GridLayoutManager(this, 2)
        rv_secrets.adapter = SecretAdapter(secrets, this)
    }

    fun addSecrets() {
        secrets.add(Pair("Test", "Hallo"))
        secrets.add(Pair("ABC", "DEF"))
    }

    /*override fun writeToParcel(parcel: Parcel, flags: Int) {
    }

    override fun describeContents(): Int {
        return 0
    }

    companion object CREATOR : Parcelable.Creator<AppActivity> {
        override fun createFromParcel(parcel: Parcel): AppActivity {
            return AppActivity(parcel)
        }

        override fun newArray(size: Int): Array<AppActivity?> {
            return arrayOfNulls(size)
        }
    }*/
}

/*
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import android.view.KeyEvent
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.Storage

class AppActivity : Activity() {

    private val common: Common = Common.getInstance(this)
    lateinit var preferences: SharedPreferences

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app)
        System.loadLibrary("etopan")

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)

        common.requestString("data/get_secure",
            { response ->
                Log.d("Test", Storage(common, response).map.toString())
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

    private fun openMain() {
        val main = Intent(this@AppActivity, LoginActivity::class.java)
        this@AppActivity.startActivity(main)
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            openMain()
            return true
        }

        return super.onKeyDown(keyCode, event)
    }
}*/