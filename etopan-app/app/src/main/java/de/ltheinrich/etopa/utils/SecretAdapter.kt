package de.ltheinrich.etopa.utils

import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import de.ltheinrich.etopa.R
import kotlinx.android.synthetic.main.secret_item.view.*

class SecretAdapter(val items: ArrayList<Pair<String, String>>, val context: Context) :
    RecyclerView.Adapter<ViewHolder>() {

    override fun getItemCount(): Int {
        return items.size
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        return ViewHolder(
            LayoutInflater.from(context).inflate(R.layout.secret_item, parent, false)
        )
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder?.name?.text = items.get(position).first
        holder?.token?.text = items.get(position).second
    }
}

class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
    val name = view.name
    val token = view.token
}