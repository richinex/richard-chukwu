// components/gallery.rs
use yew::prelude::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    html! {
        <>
            // Remove specific background and text color classes
            <div class="gallery p-4">
                <h1 class="text-3xl font-bold text-center">{ "Gallery" }</h1>
                <p class="text-center mb-4">{ "Some of my favorite projects." }</p>
                <div class="gallery-grid grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <GalleryItem
                        image_src="/richard-chukwu/images/portfolio.png"
                        title="My Portfolio"
                        description="A Personal site built with Rust and WASM."
                    />
                    <GalleryItem
                        image_src="/richard-chukwu/images/fitmyeis_01.png"
                        title="FitMyEIS"
                        description="A revolutionary new app that changes the way you do business."
                    />
                    <GalleryItem
                        image_src="/richard-chukwu/images/pymultipleis.png"
                        title="PyMultiplEIS (Jax version)"
                        description="A python package for batch fitting electrochemical impedance (EIS) data."
                    />
                    <GalleryItem
                    image_src="/richard-chukwu/images/organized_folder_tree.png"
                    title="Plexisort"
                    description="An OS agnostic folder organizer."
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
