#![allow(dead_code,unused_imports)]

mod tfhe_x;
mod bristol;
mod testkit;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use testkit::{Test,Report};

pub fn scenrio(dir: &str, out_path: &str, circuit_names: &[&str], num_of_trials: usize) -> std::io::Result<bool>
{
    let mut table = String::from(
        r#"\begin{table}[H]
\centering
\setlength{\tabcolsep}{3pt}
\caption{Our experimental results. All the elapsed times are reported in seconds.}
\label{tbl:exp_res}
\begin{tabular}{c|c|c|c}
Circuit & Gates & Standard & FreeXOR \\ \hline
"#
    );

    for circ_name in circuit_names
    {
        let path = format!("{}/{}.bf", dir, circ_name);
        let st = Test::homomorphic_std(path.as_str(), num_of_trials);
        if !st.succeeded {return Ok(false);}

        let fx = Test::homomorphic_freexor(path.as_str(), num_of_trials);
        if !fx.succeeded {return Ok(false);}

        table.push_str(&format!("{} & {} & {:.3} & {:.3} \\\\ \\hline\n",
        circ_name, fx.circuit_statistics(), st.avg_elapsed_seconds(), fx.avg_elapsed_seconds()));     
    }

    table.push_str(
        r#"\end{tabular}
\end{table}
"#
    );

    let mut file = File::create(out_path)?;
    file.write_all(table.as_bytes())?;
    return Ok(true);
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    //scenrio("samples/crypto","crypto.tex",&["AES128","AES192","AES256","SHA256","SHA512"],10)?;
    scenrio("samples/toys","toys.tex",&["XORTree","toy1","toy2","toy3"],10)?;
    Ok(())
}