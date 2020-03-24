use sdl2::Error;

pub trait SystemTrait
{
   fn boot( &self ) -> Result<(), Error>;
}