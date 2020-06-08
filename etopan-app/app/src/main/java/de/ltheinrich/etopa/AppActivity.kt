package de.ltheinrich.etopa

import android.annotation.SuppressLint
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.graphics.Bitmap
import android.net.Uri
import android.os.Bundle
import android.os.StrictMode
import android.view.KeyEvent
import android.webkit.*
import androidx.webkit.WebViewAssetLoader
import java.net.InetSocketAddress
import java.net.Socket

class AppActivity : Activity() {

    lateinit var preferences: SharedPreferences
    private lateinit var webView: WebView
    private var online = false
    private var removeLogin = false;

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
        val webViewAssetLoader = WebViewAssetLoader.Builder()
            .addPathHandler("/assets/", WebViewAssetLoader.AssetsPathHandler(this)).build()
        webView.webViewClient = object : WebViewClient() {
            override fun shouldInterceptRequest(
                view: WebView?,
                request: WebResourceRequest
            ): WebResourceResponse? {
                var url = request.url.toString().replace("#", "").replace("?", "");
                if (url.startsWith("https://appassets.androidplatform.net/assets/") && url.endsWith(
                        "/"
                    )
                ) {
                    url += "index.html";
                }
                val interceptedWebRequest =
                    webViewAssetLoader.shouldInterceptRequest(Uri.parse(url))
                interceptedWebRequest?.let {
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
                if (removeLogin) {
                    view?.evaluateJavascript("localStorage.removeItem(\"username\");", null)
                    view?.evaluateJavascript("localStorage.removeItem(\"token\");", null)
                    removeLogin = false
                }
                val key = intent.extras?.getString("key").orEmpty()
                view?.evaluateJavascript("sessionStorage.setItem(\"storage_key\", \"$key\");", null)
                if (preferences.contains("storage_data")) {
                    val storageData =
                        preferences.getString("storage_data", "").orEmpty().replace("\n", "\\n")
                    view?.evaluateJavascript(
                        "localStorage.setItem(\"storage_data\", `$storageData`.replace(\"\\\\n\", \"\\n\"));",
                        null
                    )
                }
                if (preferences.contains("token")) {
                    val token = preferences.getString("token", "").orEmpty()
                    view?.evaluateJavascript("localStorage.setItem(\"token\", \"$token\");", null)
                }
            }

            override fun onPageFinished(view: WebView?, url: String?) {
                if (url != null) {
                    val splitUrl = url.split('#')[0].split('?')[0]
                    if (splitUrl.endsWith("/index.html") || splitUrl.endsWith("/")) {
                        webView.evaluateJavascript(
                            "username.value = \"" + preferences.getString(
                                "username",
                                ""
                            ).orEmpty() + "\";password.value = \"" + preferences.getString(
                                "password",
                                ""
                            ).orEmpty() + "\";" + if (online) {
                                "setTimeout(function() {" + if (splitUrl.endsWith("/app/") || splitUrl.endsWith(
                                        "/app/index.html"
                                    )
                                ) {
                                    "offline_mode"
                                } else {
                                    "login_btn"
                                } + ".click();}, 1000);"
                            } else {
                                ""
                            }, null
                        )
                    }
                }
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"username\") != null) {Android.setUsername(localStorage.getItem(\"username\"));}}, 1000);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"token\") != null) {Android.setToken(localStorage.getItem(\"token\"));}}, 2000);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"storage_data\") != null) {Android.setStorageData(localStorage.getItem(\"storage_data\"));}}, 2250);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(sessionStorage.getItem(\"storage_key\") != null) {Android.setKey(sessionStorage.getItem(\"storage_key\"));}}, 2500);",
                    null
                )
                view?.evaluateJavascript(
                    "setTimeout(function() {if(localStorage.getItem(\"lang\") != null) {Android.setLang(localStorage.getItem(\"lang\"));}}, 2750);",
                    null
                )

            }
        }

        if (intent.extras?.containsKey("key")!!) {
            removeLogin = !preferences.contains("token")
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
                webView.loadUrl(
                    "https://$instance" + if (removeLogin) {
                        "/index.html"
                    } else {
                        "/app/index.html"
                    }
                )
                online = true
            } catch (e: Exception) {
                webView.loadUrl("https://appassets.androidplatform.net/assets/app/index.html")
            }
        } else {
            openMain()
        }
    }

    private fun openMain() {
        val main = Intent(this@AppActivity, LoginActivity::class.java)
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

        @JavascriptInterface
        fun setToken(token: String) {
            ctx.preferences.edit().putString("token", token).apply()
        }
    }
}
