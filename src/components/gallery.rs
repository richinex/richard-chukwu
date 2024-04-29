// components/gallery.rs
use yew::prelude::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    html! {
        <>
            // Remove specific background and text color classes
            <div class="gallery p-4">
                // <h1 class="text-3xl font-bold text-center">{ "Gallery" }</h1>
                <p class="text-center mb-4">{ "Pictures of some of my favorite projects." }</p>
                <div class="gallery-grid grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <GalleryItem
                        image_src="/richard-chukwu/images/richie-portfolio.png"
                        title="My Portfolio"
                        description="A Personal site built with Rust and WASM (No JavaScript)."
                    />
                    <GalleryItem
                        image_src="/richard-chukwu/images/fitmyeis_01.png"
                        title="FitMyEIS"
                        description="A web application for fitting dynamic impedance data."
                    />
                    <GalleryItem
                        image_src="/richard-chukwu/images/pymultipleis.png"
                        title="PyMultiplEIS (Jax version)"
                        description="A Python library for batch fitting electrochemical impedance (EIS) data."
                    />
                        <GalleryItem
                        image_src="/richard-chukwu/images/plexisort.png"
                        title="Plexisort"
                        description="A command-line tool designed to organize your files based on metadata."
                    />
                        <GalleryItem
                        image_src="/richard-chukwu/images/config-server.png"
                        title="Config Server"
                        description="A Rust-based configuration management server."
                    />
                    <GalleryItem
                        image_src="/richard-chukwu/images/thunderhawk.png"
                        title="ThunderHawk"
                        description="A command-line tool designed for load testing APIs."
                    />


                </div>
            </div>
        </>
    }
}


#[derive(Properties, PartialEq)]
pub struct GalleryItemProps {
    pub image_src: String,
    pub title: String,
    pub description: String,
}

#[function_component(GalleryItem)]
pub fn gallery_item(GalleryItemProps { image_src, title, description }: &GalleryItemProps) -> Html {
    html! {
        <div class="gallery-item overflow-hidden rounded-lg shadow-lg">
            <img src={image_src.clone()} alt={title.clone()} class="gallery-image w-full h-auto" />
            <div class="gallery-info p-4">
                <h3 class="text-lg font-semibold">{title}</h3>
                <p>{description}</p>
            </div>
        </div>
    }
}
