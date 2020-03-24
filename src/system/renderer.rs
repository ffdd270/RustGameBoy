extern crate sdl2;

use crate::system::system_trait::SystemTrait;
use sdl2::Error;


pub struct Renderer
{
    sdl_context : sdl2::Sdl,
}

impl SystemTrait for Renderer
{
    fn boot( &self ) -> Result<(), Error>
    {

        Ok( () )
    }
}

pub fn create_renderer(  ) -> Result<Renderer, String>
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    Ok( Renderer{ sdl_context } )

}