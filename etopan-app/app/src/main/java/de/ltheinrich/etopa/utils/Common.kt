package de.ltheinrich.etopa.utils

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.util.Log
import android.widget.Toast
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.Volley
import org.json.JSONObject
import kotlin.reflect.KClass


typealias Handler = (response: JSONObject) -> Unit

class Common constructor(context: Context) {
    fun request(
        url: String,
        handler: Handler,
        vararg data: Pair<String, String>
    ) {
        val jsonObjectRequest = object : JsonObjectRequest(
            Method.POST, url, null,
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
        val app = Intent(context, cls.java)
        for ((key, value) in extras) {
            app.putExtra(key, value)
        }
        context.startActivity(app)
    }

    fun toast(stringId: Int) {
        Toast.makeText(context, stringId, Toast.LENGTH_LONG).show()
    }

    companion object {
        @Volatile
        private var INSTANCE: Common? = null
        fun getInstance(context: Context) =
            INSTANCE ?: synchronized(this) {
                INSTANCE
                    ?: Common(context).also {
                        INSTANCE = it
                    }
            }
    }

    private val context: Context by lazy {
        context
    }

    private val http: RequestQueue by lazy {
        Volley.newRequestQueue(context.applicationContext)
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
