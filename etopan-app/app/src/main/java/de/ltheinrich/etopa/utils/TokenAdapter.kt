package de.ltheinrich.etopa.utils

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import de.ltheinrich.etopa.AppActivity
import de.ltheinrich.etopa.R
import kotlinx.android.synthetic.main.secret_item.view.*

class TokenAdapter(
    private val items: ArrayList<Pair<String, String>>,
    private val context: AppActivity
) :
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
        holder.name.text = items[position].first
        holder.token.text = items[position].second
        holder.token.setOnClickListener {
            context.common.copyToClipboard(items[position].second)
        }
    }
}

class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
    val name: TextView = view.name
    val token: Button = view.token
}