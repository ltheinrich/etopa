package de.ltheinrich.etopan

import android.annotation.SuppressLint
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.graphics.Bitmap
import android.os.Bundle
import android.os.StrictMode
import android.view.KeyEvent
import android.webkit.*
import androidx.appcompat.app.AppCompatActivity
import androidx.webkit.WebViewAssetLoader
import java.net.InetSocketAddress
import java.net.Socket

class AppActivity : AppCompatActivity() {

    lateinit var preferences: SharedPreferences
    private lateinit var webView: WebView
    private var online = false

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app)

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        val instance =
            preferences.getString("instance", getString(R.string.default_instance)).orEmpty()

        webView = findViewById(R.id.webview)
        webView.settings.javaScriptEnabled = true
        webView.settings.allowContentAccess = true
        webView.settings.allowFileAccess = true
        webView.settings.allowFileAccessFromFileURLs = true
        webView.settings.allowUniversalAccessFromFileURLs = true
        webView.settings.domStorageEnabled = true
        webView.settings.setAppCachePath(this@AppActivity.applicationContext.cacheDir.absolutePath)
        webView.settings.setAppCacheEnabled(true)
        webView.addJavascriptInterface(WebAppInterface(this), "Android")
        val webViewAssetLoader = WebViewAssetLoader.Builder().addPathHandler("/assets/", WebViewAssetLoader.AssetsPathHandler(this)).build()
        webView.webViewClient = object : WebViewClient() {
            override fun shouldInterceptRequest(
                view: WebView?,
                request: WebResourceRequest
            ): WebResourceResponse? {
                val interceptedWebRequest = webViewAssetLoader.shouldInterceptRequest(request.url)
                interceptedWebRequest?.let {
                    val url = request.url.toString()
                    if (url.endsWith("js", true)) {
                        it.mimeType = "text/javascript"
                        if (url.endsWith("/config.js") && !online) {
                            it.data = ("export const API_URL = \"https://$instance\";\n" +
                                    "export const TITLE = \"Etopa\";\n" +
                                    "export const LANG = \"en\";\n").byteInputStream()
                        }
                    } else if (url.endsWith("wasm", true)) {
                        it.mimeType = "application/wasm"
                    }
                }
                return interceptedWebRequest
            }

            override fun onPageStarted(view: WebView?, url: String?, favicon: Bitmap?) {
                val key = intent.extras?.getString("key").orEmpty()
                view?.evaluateJavascript("sessionStorage.setItem(\"storage_key\", \"$key\");",null)
                if(preferences.contains("storage_data")) {
                    val storageData = preferences.getString("storage_data", "").orEmpty().replace("\n", "\\n")
                    val a = "localStorage.setItem(\"storage_data\", `$storageData`.replace(\"\\\\n\", \"\\n\"));"
                    println(a)
                    view?.evaluateJavascript(a, null)
                }
            }

            override fun onPageFinished(view: WebView?, url: String?) {
                if(url != null) {
                    val splitUrl = url.split('#')[0].split('?')[0]
                    if (splitUrl.endsWith("/index.html") || splitUrl.endsWith("/")) {
                        webView.evaluateJavascript(
                            "username.value = \"" + preferences.getString(
                                "username",
                                ""
                            ).orEmpty() + "\";password.value = \"" + preferences.getString(
                                "password",
                                ""
                            ).orEmpty() + "\";setTimeout(function() {" + if (splitUrl.endsWith("/app/") || splitUrl.endsWith("/app/index.html")) { "offline_mode" } else { "login_btn"} + ".click();}, 1000);", null
                        )
                    }
                }
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"username\") != null) {Android.setUsername(localStorage.getItem(\"username\"));}}, 2000);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(sessionStorage.getItem(\"storage_key\") != null) {Android.setKey(sessionStorage.getItem(\"storage_key\"));}}, 2000);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"storage_data\") != null) {Android.setStorageData(localStorage.getItem(\"storage_data\"));}}, 2000);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"lang\") != null) {Android.setLang(localStorage.getItem(\"lang\"));}}, 2000);",
                    null
                )
            }
        }

        if (intent.extras?.containsKey("key")!!) {
            val policy =
                StrictMode.ThreadPolicy.Builder().permitAll().build()
            StrictMode.setThreadPolicy(policy)
            try {
                val split = instance.split(":")
                val addr = if (split.size > 1) {
                    InetSocketAddress(split[0], split[1].toInt())
                } else {
                    InetSocketAddress(instance, 443)
                }
                Socket().connect(addr, 3000)
                webView.loadUrl("https://$instance")
                online = true
            } catch (e: Exception) {
                webView.loadUrl("https://appassets.androidplatform.net/assets/app/index.html")
            }
        } else {
            openMain()
        }
    }

    private fun openMain() {
        val main = Intent(this@AppActivity, MainActivity::class.java)
        this@AppActivity.startActivity(main)
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK && webView.canGoBack()) {
            webView.goBack()
            return true
        }

        return super.onKeyDown(keyCode, event)
    }

    class WebAppInterface(private val ctx: AppActivity) {
        @JavascriptInterface
        fun setUsername(username: String) {
            ctx.preferences.edit().putString("username", username).apply()
        }

        @JavascriptInterface
        fun setKey(key: String) {
            ctx.preferences.edit().putString("key", key).apply()
        }

        @JavascriptInterface
        fun setStorageData(storage_data: String) {
            ctx.preferences.edit().putString("storage_data", storage_data).apply()
        }

        @JavascriptInterface
        fun setLang(lang: String) {
            ctx.preferences.edit().putString("lang", lang).apply()
        }
    }
}
