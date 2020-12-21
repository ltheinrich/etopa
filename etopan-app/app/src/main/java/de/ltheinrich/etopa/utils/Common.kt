package de.ltheinrich.etopa.utils

import android.app.Activity
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Intent
import android.content.SharedPreferences
import android.util.Log
import android.view.*
import android.view.inputmethod.InputMethodManager
import android.widget.Toast
import androidx.core.content.ContextCompat
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.VolleyError
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.StringRequest
import com.android.volley.toolbox.Volley
import com.google.android.material.textfield.TextInputLayout
import de.ltheinrich.etopa.*
import org.json.JSONObject
import java.util.*
import kotlin.reflect.KClass

typealias Handler = (response: JSONObject) -> Unit
typealias StringHandler = (response: String) -> Unit
typealias ErrorHandler = (error: VolleyError) -> Unit

const val emptyPin = "******"
const val emptyPinHash = "8326de6693e2dc5e15d9d2031d26844c"

const val emptyPassword = "************"
const val emptyPasswordHash = "08d299150597a36973bf282c1ce59602eaa12c3607d3034d7ea29bb64710d65c"
const val emptyKeyHash = "c353cdd4c437c0dc01d6378525e25c1d"

var library: Boolean = false

fun inputString(inputLayout: TextInputLayout): String {
    return inputLayout.editText?.text.toString()
}

class Common constructor(activity: Activity) {

    lateinit var instance: String
    lateinit var username: String
    lateinit var passwordHash: String
    lateinit var keyHash: String
    lateinit var pinHash: String
    lateinit var token: String
    lateinit var storage: Storage
    lateinit var backActivity: Class<*>
    var offline: Boolean = false
    var extendedMenu: Boolean = false

    fun handleMenu(item: MenuItem) = when (item.itemId) {
        R.id.action_add -> {
            openActivity(AddActivity::class)
            true
        }
        R.id.action_settings -> {
            openActivity(SettingsActivity::class)
            true
        }
        R.id.action_licenses -> {
            openActivity(LicensesActivity::class)
            true
        }
        android.R.id.home -> {
            openActivity(backActivity)
            true
        }
        else -> {
            false
        }
    }

    fun backKey(keyCode: Int): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            openActivity(backActivity)
            return true
        }
        return false
    }

    fun createMenu(menu: Menu?): Boolean {
        activity.menuInflater.inflate(R.menu.toolbar_menu, menu)
        val itemIds = arrayOf(R.id.action_add, R.id.action_settings)
        itemIds.forEach { itemId ->
            val item = menu?.findItem(itemId)
            if (item != null) {
                item.isVisible = extendedMenu
            }
        }
        return true
    }

    fun decryptLogin(preferences: SharedPreferences) {
        instance = preferences.getString("instance", encrypt(pinHash, "etopa.de"))?.let {
            decrypt(
                pinHash,
                it
            )
        }.toString()
        username = decrypt(
            pinHash,
            preferences.getString("username", encrypt(pinHash, "")).orEmpty()
        )
        passwordHash = decrypt(
            pinHash,
            preferences.getString("passwordHash", encrypt(pinHash, "")).orEmpty()
        )
        keyHash = decrypt(
            pinHash,
            preferences.getString("keyHash", encrypt(pinHash, "")).orEmpty()
        )
        token = decrypt(
            pinHash,
            preferences.getString("token", encrypt(pinHash, "")).orEmpty()
        )
    }

    fun encryptLogin(preferences: SharedPreferences, pinHash: String) {
        val editor = preferences.edit()
        val defaultInstance = activity.getString(R.string.default_instance)

        if (instance.isEmpty() || instance == defaultInstance) {
            editor.remove("instance")
            instance = defaultInstance
        } else {
            editor.putString("instance", encrypt(pinHash, instance))
        }

        if (passwordHash == emptyPasswordHash) {
            passwordHash = decrypt(
                this.pinHash,
                preferences.getString("passwordHash", encrypt(this.pinHash, "")).orEmpty()
            )
        }

        if (keyHash == emptyKeyHash) {
            keyHash = decrypt(
                this.pinHash,
                preferences.getString("keyHash", encrypt(this.pinHash, "")).orEmpty()
            )
        }

        editor.putString("username", encrypt(pinHash, username))
        editor.putString("passwordHash", encrypt(pinHash, passwordHash))
        editor.putString("keyHash", encrypt(pinHash, keyHash))
        val secretStorage = preferences.getString("secretStorage", null)
        if (secretStorage != null) {
            editor.putString(
                "secretStorage",
                encrypt(pinHash, decrypt(this.pinHash, secretStorage))
            )
        }

        editor.remove(token)
        setPin(editor, pinHash)
    }

    fun setPin(editor: SharedPreferences.Editor, pinHash: String) {
        val splitAt = Random().nextInt(30)
        val uuid = UUID.randomUUID().toString()
        val pinSetEncrypted =
            encrypt(
                pinHash,
                uuid.substring(0, splitAt) + "etopan_pin_set" + uuid.substring(splitAt)
            )

        editor.putString("pin_set", pinSetEncrypted)
        editor.apply()

        this.pinHash = pinHash
    }

    fun newLogin(preferences: SharedPreferences) {
        request("user/login",
            { response ->
                if (response.has("token")) {
                    token = response.getString("token")
                    val editor = preferences.edit()
                    editor.putString("token", encrypt(pinHash, token))
                    editor.apply()
                    openActivity(AppActivity::class)
                } else {
                    toast(R.string.incorrect_login)
                    openActivity(SettingsActivity::class, Pair("incorrectLogin", "incorrectLogin"))
                }
            },
            Pair("username", username),
            Pair("password", passwordHash),
            error_handler = { offlineLogin(preferences) })
    }

    fun offlineLogin(preferences: SharedPreferences) {
        toast(R.string.network_unreachable)
        if (preferences.contains("secretStorage")) {
            openActivity(AppActivity::class)
        }
    }

    fun request(
        url: String,
        handler: Handler,
        vararg data: Pair<String, String>,
        error_handler: ErrorHandler = { error: VolleyError ->
            Log.e(
                "HTTP Request",
                error.toString()
            )
        },
    ) {
        val jsonObjectRequest = object : JsonObjectRequest(
            Method.POST, "https://$instance/$url", null,
            Response.Listener { response ->
                offline = false
                handler(response)
            },
            Response.ErrorListener { error ->
                offline = true
                Log.d("Network error", error.toString())
                error_handler(error)
            }
        ) {
            override fun getHeaders(): Map<String, String> {
                return data.toMap()
            }
        }
        http.add(jsonObjectRequest)
    }

    fun requestString(
        url: String,
        handler: StringHandler,
        vararg data: Pair<String, String>,
        error_handler: ErrorHandler = { error: VolleyError ->
            Log.e(
                "HTTP Request",
                error.toString()
            )
        },
    ) {
        val stringRequest = object : StringRequest(
            Method.POST, "https://$instance/$url",
            Response.Listener { response ->
                offline = false
                handler(response)
            },
            Response.ErrorListener { error ->
                offline = true
                error_handler(error)
            }
        ) {
            override fun getHeaders(): Map<String, String> {
                return data.toMap()
            }
        }
        http.add(stringRequest)
    }

    fun <T : Activity> openActivity(
        cls: KClass<T>,
        vararg extras: Pair<String, String>,
    ) {
        val app = Intent(activity, cls.java)
        for ((key, value) in extras) {
            app.putExtra(key, value)
        }
        activity.startActivity(app)
    }

    private fun openActivity(
        cls: Class<*>,
        vararg extras: Pair<String, String>,
    ) {
        val app = Intent(activity, cls)
        for ((key, value) in extras) {
            app.putExtra(key, value)
        }
        activity.startActivity(app)
    }

    fun toast(stringId: Int, height: Int = 0, length: Int = Toast.LENGTH_LONG) {
        val toast = Toast.makeText(activity, stringId, length)
        if (height != 0)
            toast.setGravity(Gravity.TOP or Gravity.CENTER_HORIZONTAL, 0, height)
        toast.show()
    }

    fun hideKeyboard(activity: Activity) {
        val imm: InputMethodManager =
            activity.getSystemService(Activity.INPUT_METHOD_SERVICE) as InputMethodManager
        var view: View? = activity.currentFocus
        if (view == null) {
            view = View(activity)
        }
        imm.hideSoftInputFromWindow(view.windowToken, 0)
    }

    fun copyToClipboard(toCopy: String) {
        val clipboard = ContextCompat.getSystemService(
            activity,
            ClipboardManager::class.java
        )
        val clip = ClipData.newPlainText(toCopy, toCopy)
        clipboard?.setPrimaryClip(clip)
    }

    companion object {
        @Volatile
        private var INSTANCE: Common? = null
        fun getInstance(activity: Activity): Common =
            if (library) {
                INSTANCE ?: synchronized(this) {
                    INSTANCE
                        ?: Common(activity).also {
                            INSTANCE = it
                        }
                }
            } else {
                System.loadLibrary("etopan")
                library = true
                getInstance(activity)
            }
    }

    private val activity: Activity by lazy {
        activity
    }

    private val http: RequestQueue by lazy {
        Volley.newRequestQueue(activity.applicationContext)
    }

    external fun hashKey(key: String): String

    external fun hashPassword(password: String): String

    external fun hashPin(pin: String): String

    external fun hashName(name: String): String

    external fun hashArgon2Hashed(passwordHash: String): String

    external fun encrypt(key: String, data: String): String

    external fun decrypt(key: String, data: String): String

    external fun generateToken(secret: String): String

    /*val imageLoader: ImageLoader by lazy {
        ImageLoader(requestQueue,
            object : ImageLoader.ImageCache {
                private val cache = LruCache<String, Bitmap>(20)
                override fun getBitmap(url: String): Bitmap {
                    return cache.get(url)
                }

                override fun putBitmap(url: String, bitmap: Bitmap) {
                    cache.put(url, bitmap)
                }
            })
    }*/
}