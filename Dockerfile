FROM fedora:latest

RUN dnf update -y
RUN dnf install git xorg-x11-server-Xvfb procps-ng wget libjpeg-turbo-devel 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4 from the latest release
ADD https://download.gnome.org/sources/gtk/4.6/gtk-4.6.0.tar.xz /tmp/gtk-4.6.0.tar.xz
RUN tar -xf /tmp/gtk-4.6.0.tar.xz --directory /tmp
WORKDIR /tmp/gtk-4.6.0
RUN meson _build --prefix=/usr -Dgtk_doc=false -Dintrospection=disabled -Dbuild-examples=false -Dbuild-tests=false -Ddemos=false
RUN ninja -C _build
RUN ninja -C _build install

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
ENV PATH="$HOME/.cargo/bin/:${PATH}"
RUN rustup toolchain install stable --profile minimal --allow-downgrade -c clippy -c rustfmt
RUN rustup toolchain install beta --profile minimal --allow-downgrade -c clippy -c rustfmt
RUN rustup toolchain install nightly --profile minimal --allow-downgrade -c clippy -c rustfmt
RUN rustup toolchain install 1.56.0 --profile minimal --allow-downgrade -c clippy -c rustfmt

RUN rustup default stable
