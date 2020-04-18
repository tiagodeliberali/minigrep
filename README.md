# Minigrep

Exercise of [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) from the book The Rust Programming Language

Added clap, STDIN and Regex from [Chapter 2](https://livebook.manning.com/book/rust-in-action/chapter-2/) of the book Rust in Action

## Using minigrep

If you just run `minigrep`, you can get some help from `clap`:

<pre><font color="#A6E22E"><b>➜  </b></font><font color="#A1EFE4"><b>minigrep</b></font> <font color="#66D9EF"><b>git:(</b></font><font color="#F92672"><b>master</b></font><font color="#66D9EF"><b>)</b></font> minigrep
<font color="#F92672"><b>error:</b></font> The following required arguments were not provided:
    <font color="#F92672"><b>&lt;pattern&gt;</b></font>

USAGE:
    minigrep &lt;pattern&gt; [input]

For more information try <font color="#555753">--help</font>
</pre>

You can use minigrep from STDIN:

<pre><font color="#A6E22E"><b>➜  </b></font><font color="#A1EFE4"><b>minigrep</b></font> <font color="#66D9EF"><b>git:(</b></font><font color="#F92672"><b>master</b></font><font color="#66D9EF"><b>)</b></font> cat poema.txt | minigrep pedra      
No meio do caminho tinha uma pedra
tinha uma pedra no meio do caminho
tinha uma pedra
no meio do caminho tinha uma pedra.
tinha uma pedra
tinha uma pedra no meio do caminho
no meio do caminho tinha uma pedra.</pre>

Or pass a filename to it:

<pre><font color="#A6E22E"><b>➜  </b></font><font color="#A1EFE4"><b>minigrep</b></font> <font color="#66D9EF"><b>git:(</b></font><font color="#F92672"><b>master</b></font><font color="#66D9EF"><b>)</b></font> minigrep esquecerei poema.txt
Nunca me esquecerei desse acontecimento
Nunca me esquecerei que no meio do caminho
</pre>

Also, you can use regex to search:

<pre><font color="#A6E22E"><b>➜  </b></font><font color="#A1EFE4"><b>minigrep</b></font> <font color="#66D9EF"><b>git:(</b></font><font color="#F92672"><b>master</b></font><font color="#66D9EF"><b>)</b></font> cat poema.txt | minigrep &apos;tinha(\D+)caminho&apos;
tinha uma pedra no meio do caminho
tinha uma pedra no meio do caminho</pre>

You can get some help:

<pre><font color="#A6E22E"><b>➜  </b></font><font color="#A1EFE4"><b>minigrep</b></font> <font color="#66D9EF"><b>git:(</b></font><font color="#F92672"><b>master</b></font><font color="#66D9EF"><b>)</b></font> minigrep --help
minigrep 0.1
Busca por padrões em arquivos no ou STDIN

USAGE:
    minigrep &lt;pattern&gt; [input]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    &lt;pattern&gt;    O padrão a ser buscado. Pode user regex. Exemplo de busca case insensitive: &apos;(?i)pEDrA&apos;.
    &lt;input&gt;      O arquivo usado como referência
</pre>
