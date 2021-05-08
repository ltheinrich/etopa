package de.ltheinrich.etopa.utils

class Storage(private val common: Common, data: String) {

    val map = LinkedHashMap<String, String>()

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

        val sort = splitLines["secrets_sort"].orEmpty().split(',');
        sort.forEach { nameHash ->
            val name = splitLines[nameHash + "_secret_name"]
            val secret = splitLines[nameHash + "_secret"]
            if (name != null && secret != null)
                map[name] = secret
        }

        names.forEach { nameHash ->
            if (!sort.contains(nameHash)) {
                val name = splitLines[nameHash + "_secret_name"]
                val secret = splitLines[nameHash + "_secret"]
                if (name != null && secret != null)
                    map[name] = secret
            }
        }
    }

    fun encrypt(keyHash: String): String {
        val secureStorage = StringBuilder()
        val sort = StringBuilder()
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
            sort.append(hashedName)
            sort.append(',')
        }
        return secureStorage.toString()
    }
}
