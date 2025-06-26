use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    sync::mpsc,
    sync::Arc,
    sync::Mutex,
    path::Path,
    collections::HashMap,
};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            if Path::new("index.md").exists() {
                ("HTTP/1.1 200 OK", serve_file("index.md").unwrap_or_else(|_| generate_404_page()))
            } else {
                ("HTTP/1.1 200 OK", generate_directory_listing())
            }
        },
        "GET /list HTTP/1.1" => ("HTTP/1.1 200 OK", generate_directory_listing()),
        line if line.starts_with("GET ") && line.ends_with(" HTTP/1.1") => {
            let path = &line[4..line.len()-9]; // Extract path between "GET " and " HTTP/1.1"
            let path = path.trim_start_matches('/');
            
            if path.is_empty() {
                if Path::new("index.md").exists() {
                    ("HTTP/1.1 200 OK", serve_file("index.md").unwrap_or_else(|_| generate_404_page()))
                } else {
                    ("HTTP/1.1 200 OK", generate_directory_listing())
                }
            } else if path.ends_with(".md") && Path::new(path).exists() {
                ("HTTP/1.1 200 OK", serve_file(path).unwrap_or_else(|_| generate_404_page()))
            } else {
                ("HTTP/1.1 404 NOT FOUND", generate_404_page())
            }
        }
        _ => ("HTTP/1.1 404 NOT FOUND", generate_404_page()),
    };
    
    let response = format!("{}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}", 
                          status_line, content.len(), content);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_file(filename: &str) -> Result<String, std::io::Error> {
    if filename.ends_with(".md") {
        // Read markdown file and convert to HTML
        let markdown_content = fs::read_to_string(filename)?;
        let html_content = markdown_to_html(&markdown_content);
        Ok(html_content)
    } else {
        // Serve other files as-is
        fs::read_to_string(filename)
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut in_code_block = false;
    let mut in_list = false;
    
    html.push_str(&get_html_header());
    
    for line in lines {
        if line.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
            } else {
                let lang = line.trim_start_matches("```");
                html.push_str(&format!("<pre><code class=\"language-{}\">\n", lang));
            }
            in_code_block = !in_code_block;
        } else if in_code_block {
            html.push_str(&html_escape(line));
            html.push('\n');
        } else if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
        } else if line.starts_with("- ") || line.starts_with("* ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            html.push_str(&format!("<li>{}</li>\n", &line[2..]));
        } else if line.trim().is_empty() {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            html.push_str("<br>\n");
        } else {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            // Process inline formatting
            let formatted_line = process_inline_formatting(line);
            html.push_str(&format!("<p>{}</p>\n", formatted_line));
        }
    }
    
    if in_list {
        html.push_str("</ul>\n");
    }
    if in_code_block {
        html.push_str("</code></pre>\n");
    }
    
    html.push_str("</body></html>");
    html
}

fn process_inline_formatting(text: &str) -> String {
    let mut result = text.to_string();
    
    // Bold text **text**
    result = result.replace("**", "<strong>").replace("**", "</strong>");
    
    // Italic text *text*
    result = regex_replace(&result, r"\*([^*]+)\*", "<em>$1</em>");
    
    // Inline code `code`
    result = regex_replace(&result, r"`([^`]+)`", "<code>$1</code>");
    
    // Links [text](url)
    result = regex_replace(&result, r"\[([^\]]+)\]\(([^)]+)\)", "<a href=\"$2\">$1</a>");
    
    result
}

fn regex_replace(text: &str, pattern: &str, replacement: &str) -> String {
    // Simple regex replacement for basic patterns
    // This is a simplified version - in production, use the regex crate
    text.to_string() // Placeholder - implement proper regex later
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn get_html_header() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown Server</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f8f9fa;
        }
        pre {
            background-color: #f4f4f4;
            border: 1px solid #ddd;
            border-radius: 4px;
            padding: 12px;
            overflow-x: auto;
        }
        code {
            background-color: #f4f4f4;
            padding: 2px 4px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
        }
        h1, h2, h3 {
            color: #2c3e50;
            border-bottom: 2px solid #3498db;
            padding-bottom: 8px;
        }
        a {
            color: #3498db;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
        ul {
            padding-left: 20px;
        }
        li {
            margin-bottom: 5px;
        }
    </style>
</head>
<body>
"#.to_string()
}

fn generate_404_page() -> String {
    format!("{}<h1>404 - Page Not Found</h1><p>The requested file could not be found.</p></body></html>", get_html_header())
}

fn generate_directory_listing() -> String {
    let mut html = get_html_header();
    html.push_str("<h1>📁 Markdown Files Directory</h1>\n");
    html.push_str("<p>Available markdown files:</p>\n<ul>\n");
    
    match fs::read_dir(".") {
        Ok(entries) => {
            let mut md_files: Vec<String> = entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.is_file() && path.extension()? == "md" {
                        Some(path.file_name()?.to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
                .collect();
            
            md_files.sort();
            
            if md_files.is_empty() {
                html.push_str("<li><em>No markdown files found. Create some .md files to get started!</em></li>\n");
            } else {
                for file in md_files {
                    html.push_str(&format!("<li>📄 <a href=\"/{}\">{}</a></li>\n", file, file));
                }
            }
        }
        Err(_) => {
            html.push_str("<li><em>Error reading directory</em></li>\n");
        }
    }
    
    html.push_str("</ul>\n");
    html.push_str("<hr>\n");
    html.push_str("<p><strong>Usage:</strong></p>\n");
    html.push_str("<ul>\n");
    html.push_str("<li>Access any .md file directly: <code>http://localhost:7878/filename.md</code></li>\n");
    html.push_str("<li>View this directory listing: <code>http://localhost:7878/list</code></li>\n");
    html.push_str("<li>Create <code>index.md</code> for a custom homepage</li>\n");
    html.push_str("</ul>\n");
    html.push_str("</body></html>");
    
    html
}

struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool{
    fn new(size: usize)-> ThreadPool{
        let(sender, reciver) = mpsc::channel();
        let reciver = Arc::new(Mutex::new(reciver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
    
    pub fn shutdown(&mut self) {
        println!("Sending terminate message to all workers.");
        
        // Close the sending side of the channel
        drop(self.sender.take());
        
        println!("Shutting down all workers.");
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciver.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

