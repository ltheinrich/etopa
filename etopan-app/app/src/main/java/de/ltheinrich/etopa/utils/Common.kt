package de.ltheinrich.etopa.utils

import android.content.Context
import android.util.Log
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.toolbox.JsonObjectRequest
import com.android.volley.toolbox.Volley
import org.json.JSONObject

typealias Handler = (response: JSONObject) -> Unit

class Common constructor(context: Context) {
    
    fun login(username: String, password: String, handler: Handler) {
        requestRaw(
            "https://etopa.de/user/login", handler,
            Pair("username", username), Pair("password", hashPassword(password))
        )
    }

    private fun requestRaw(
        url: String,
        handler: (JSONObject) -> Unit,
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

    private val http: RequestQueue by lazy {
        Volley.newRequestQueue(context.applicationContext)
    }

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
