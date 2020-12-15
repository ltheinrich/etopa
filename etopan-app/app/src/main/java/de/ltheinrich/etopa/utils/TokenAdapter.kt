package de.ltheinrich.etopa.utils

import android.view.LayoutInflater
import android.view.ViewGroup
import android.widget.Button
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import de.ltheinrich.etopa.AppActivity
import de.ltheinrich.etopa.databinding.SecretItemBinding

class TokenAdapter(
    private val items: ArrayList<Pair<String, String>>,
    private val context: AppActivity
) :
    RecyclerView.Adapter<ViewHolder>() {
    private lateinit var binding: SecretItemBinding

    override fun getItemCount(): Int {
        return items.size
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val inflater = LayoutInflater.from(parent.context)
        binding = SecretItemBinding.inflate(inflater, parent, false)
        return ViewHolder(binding)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.name.text = items[position].first
        holder.token.text = items[position].second
        holder.token.setOnClickListener {
            context.common.copyToClipboard(items[position].second)
        }
    }
}

class ViewHolder(binding: SecretItemBinding) : RecyclerView.ViewHolder(binding.root) {
    val name: TextView = binding.name
    val token: Button = binding.token
}