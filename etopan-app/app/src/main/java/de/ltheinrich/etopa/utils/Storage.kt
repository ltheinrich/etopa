package de.ltheinrich.etopa.utils

class Storage(private val common: Common, val data: String) {

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
}