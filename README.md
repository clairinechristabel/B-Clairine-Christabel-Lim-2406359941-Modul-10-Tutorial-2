# Tutorial 2: Broadcast Chat

## Experiment 2.1: Original code, and how it run

### How to run
1. Start the server:
   ```bash
   cargo run --bin server
   ```
2. Open three different terminals and run the client in each:
   ```bash
   cargo run --bin client
   ```

### What happens
Ketika kita menjalankan `cargo run --bin server`, server akan mulai berjalan dan *listen* di *port* 2000. 
Kemudian saat kita menjalankan tiga `cargo run --bin client` di terminal yang berbeda, masing-masing *client* akan terhubung ke server tersebut via WebSocket.

Karena menggunakan `tokio::sync::broadcast`, server memiliki sebuah *broadcast channel*. Ketika salah satu *client* mengetik dan mengirimkan sebuah pesan:
1. *Client* mengirim pesan tersebut ke *server*.
2. *Server* menerima pesan, lalu menambahkan alamat IP & *port* pengirim di depan pesan (misal: `127.0.0.1:51234: halo semuanya`).
3. *Server* kemudian me-*broadcast* pesan tersebut ke saluran (*channel*) utama.
4. Semua *client* yang terhubung (termasuk *client* yang mengirim pesan tersebut) akan menerima pesan dari *server* dan menampilkannya secara instan di layar terminal mereka masing-masing.

Hal ini dapat terjadi secara konkuren (bersamaan) tanpa saling menunggu *input* karena kita menggunakan **asynchronous programming** dengan `tokio::select!` pada masing-masing *client* dan *server*.

### Screenshots
![screenshot_terminal](image.png)