package de.ltheinrich.etopa.utils

class Storage(private val common: Common, data: String) {

    var map = LinkedHashMap<String, String>()

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

        val sort = splitLines["secrets_sort"].orEmpty().split(',')
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

    fun isLastSorted(name: String): Boolean {
        val keys = ArrayList(map.keys)
        val index = keys.indexOf(name)
        return index == keys.size - 1
    }

    fun isFirstSorted(name: String): Boolean {
        val keys = ArrayList(map.keys)
        val index = keys.indexOf(name)
        return index == 0
    }

    fun moveUp(name: String) = move(name, -1)
    fun moveDown(name: String) = move(name, 1)

    private fun move(name: String, dif: Int) {
        val keys = ArrayList(map.keys)
        val values = ArrayList(map.values)
        val index = keys.indexOf(name)
        if (index + dif < 0 || index + dif > keys.size - 1)
            return

        val other = Pair(keys[index + dif], values[index + dif])
        keys[index + dif] = keys[index]
        values[index + dif] = values[index]
        keys[index] = other.first
        values[index] = other.second

        val map = LinkedHashMap<String, String>()
        keys.forEachIndexed { i, key -> map[key] = values[i] }
        this.map = map
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
        val encryptedSort = common.encrypt(keyHash, sort.toString())
        secureStorage.append("secrets_sort=")
        secureStorage.appendLine(encryptedSort)
        return secureStorage.toString()
    }

    fun encryptSort(): String {
        val sort = StringBuilder()
        map.entries.forEach { (name, _) ->
            val hashedName = common.hashName(name)
            sort.append(hashedName)
            sort.append(',')
        }
        return common.encrypt(common.keyHash, sort.toString())
    }
}

