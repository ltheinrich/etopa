package de.ltheinrich.etopa.utils

class Storage(private val common: Common, data: String) {

    val map = HashMap<String, String>()

    init {
        val lines = data.split('\n')
        val splitLines = HashMap<String, String>()
        val names = ArrayList<String>()

        lines.forEach { line ->
            if (line.contains('=') || line.contains('_')) {
                val splitLine = line.split('=')
                splitLines[splitLine[0]] = common.decrypt(common.keyHash, splitLine[1])
                names.add(splitLine[0].split('_')[0])
            }
        }

        names.forEach { nameHash ->
            val name = splitLines[nameHash + "_secret_name"].orEmpty()
            val secret = splitLines[nameHash + "_secret"].orEmpty()
            map[name] = secret
        }
    }

    fun encrypt(keyHash: String): String {
        var secureStorage = StringBuilder()
        map.entries.forEach { (name, secret) ->
            val hashedName = common.hashName(name)
            val encryptedName = common.encrypt(keyHash, name)
            val encryptedSecret = common.encrypt(keyHash, secret)
            secureStorage.append(hashedName)
            secureStorage.append("_secret=")
            secureStorage.appendLine(encryptedSecret)
            secureStorage.append(hashedName)
            secureStorage.append("_secret_name=")
            secureStorage.appendLine(encryptedName)
        }
        return secureStorage.toString()
    }
}