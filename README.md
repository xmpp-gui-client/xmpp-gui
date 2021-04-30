# xmpp-gui

## what is this?
This is a XMPP GUI client proejct for `CS510/410: Rust Programming`.


## Requirements:
freetype2: `sudo apt install libfreetype6-dev`


## configurations & issues:
fontconfig: https://askubuntu.com/questions/1098809/applications-load-slowly-on-ubuntu-18-10-seems-related-to-fontconfig 

OpenFire: 
* docs: http://www.igniterealtime.org/projects/openfire/documentation.jsp 
* guide: https://download.igniterealtime.org/openfire/docs/latest/documentation/install-guide.html
* Downloads: http://www.igniterealtime.org/downloads/ 

Openssl: https://users.rust-lang.org/t/cargo-dependency-issue-with-openssl/42130/2 
* crates depend on openssl v0.9.24:
    * native-tls v0.2.7
    * sasl v0.3.0