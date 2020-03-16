mod declare;


trait CartridgeTrait
{
    fn load_memory(&self);
}

struct Cartridge
{

}

impl CartridgeTrait for Cartridge {
    fn load_memory( &self )
    {
    }
}