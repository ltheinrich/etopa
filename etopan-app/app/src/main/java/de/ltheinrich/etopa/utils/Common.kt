package de.ltheinrich.etopa.utils

import android.app.Activity
import android.content.Intent
import android.util.Log
import android.view.Gravity
import android.view.View
import android.view.inputmethod.InputMethodManager
import android.widget.Toast
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.Volley
import org.json.JSONObject
import kotlin.reflect.KClass

typealias Handler = (response: JSONObject) -> Unit

class Common constructor(activity: Activity) {
    lateinit var instance: String
    lateinit var username: String
    lateinit var passwordHash: String
    lateinit var keyHash: String
    lateinit var pinHash: String
    lateinit var token: String

    fun request(
        url: String,
        handler: Handler,
        vararg data: Pair<String, String>
    ) {
        val jsonObjectRequest = object : JsonObjectRequest(
            Method.POST, "https://$instance/$url", null,
            Response.Listener { response -> handler(response) },
            Response.ErrorListener { error -> Log.e("HTTP Request", error.toString()) }
        ) {
            override fun getHeaders(): Map<String, String> {
                return data.toMap()
            }
        }
        http.add(jsonObjectRequest)
    }

    fun <T : Activity> openActivity(
        cls: KClass<T>,
        vararg extras: Pair<String, String>
    ) {
        val app = Intent(activity, cls.java)
        for ((key, value) in extras) {
            app.putExtra(key, value)
        }
        activity.startActivity(app)
    }

    fun toast(stringId: Int, height: Int = 0) {
        val toast = Toast.makeText(activity, stringId, Toast.LENGTH_LONG)
        if (height != 0)
            toast.setGravity(Gravity.TOP or Gravity.CENTER_HORIZONTAL, 0, height)
        toast.show()
    }

    fun hideKeyboard() {
        val imm: InputMethodManager =
            activity.getSystemService(Activity.INPUT_METHOD_SERVICE) as InputMethodManager
        var view: View? = activity.currentFocus
        if (view == null) {
            view = View(activity)
        }
        imm.hideSoftInputFromWindow(view.windowToken, 0)
    }

    companion object {
        @Volatile
        private var INSTANCE: Common? = null
        fun getInstance(activity: Activity) =
            INSTANCE ?: synchronized(this) {
                INSTANCE
                    ?: Common(activity).also {
                        INSTANCE = it
                    }
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

    external fun encrypt(key: String, data: String): String

    external fun decrypt(key: String, data: String): String

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
