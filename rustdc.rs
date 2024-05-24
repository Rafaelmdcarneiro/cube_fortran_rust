#![no_std]

extern "C" {
    // GLdc C library Bindings
    fn glBegin(mode: u32);
    fn glBindTexture(target: u32, texture: u32);
    fn glEnable(cap: u32);
    fn glEnd();
    fn glClear(mask: u32);
    fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    fn glClearDepth(depth: f32);
    fn glColor3f(red: f32, green: f32, blue: f32);
    fn glCompressedTexImage2DARB(target: u32, level: i32, internalformat: u32,
                                 width: u32, height: u32, border: i32,
                                 image_size: u32, data: *const u8);
    fn glDeleteTextures(n: u32, textures: *const u32);
    fn glDepthFunc(func: u32);
    fn glGenTextures(n: u32, textures: *const u32);
    fn glKosInit();
    fn glKosSwapBuffers();
    fn glLoadIdentity();
    fn glMatrixMode(mode: u32);
    fn glRotatef(angle: f32, x: f32, y: f32, z: f32);
    fn glShadeModel(mode: u32);
    fn glTexCoord2f(s: f32, t: f32);
    fn glTranslatef(x: f32, y: f32, z: f32);
    fn glVertex3f(x: f32, y: f32, z: f32);
    fn gluPerspective(fovy: f32, aspect: f32, zNear: f32, zFar: f32);

    // KallistiOS C library bindings
    fn maple_enum_type(n: i32, func: u32) -> *mut maple_device;
    fn maple_dev_status(dev: *mut maple_device) -> *mut cont_state;
    fn printf(str: &str);

    // Fortran helper functions
    fn is_null(ptr: *mut u8) -> bool;
    fn data_size(start: &u8, end: &u8) -> u32;

    // Ferris and GCC textures linked from vqenc output
    static ferris: u8;
    static ferris_end: u8;
    static gcc: u8;
    static gcc_end: u8;
    static claw: u8;
    static claw_end: u8;
}

// KallistiOS controller constant
const MAPLE_FUNC_CONTROLLER: u32             = 0x01000000;

// maple_device represents a Dreamcast controller peripheral
#[repr(C)]
struct maple_device {
    valid: i32,
    port: i32,
    unit: i32,
    functions: u32,
    function_data: [u32; 3],
    area_code: u8,
    connector_direction: u8,
    product_name: [u8; 30],
    product_license: [u8; 60],
    standby_power: u16,
    max_power: u16,
}

// cont_state represents the state of the controller buttons
#[repr(C)]
struct cont_state {
    buttons: u32,
    ltrig: i32,
    rtrig: i32,
    joyx: i32,
    joyy: i32,
    joy2x: i32,
    joy2y: i32,
}

// Define the OpenGL constants we'll be using
const GL_TEXTURE_2D: u32                     = 0x0001;
const GL_COMPRESSED_RGB_565_VQ_TWID_KOS: u32 = 0xeee8;
const GL_COLOR_BUFFER_BIT: u32               = 0x4000;
const GL_DEPTH_BUFFER_BIT: u32               = 0x0100;
const GL_QUADS: u32                          = 0x0007;
const GL_PROJECTION: u32                     = 0x1701;
const GL_MODELVIEW: u32                      = 0x1700;
const GL_SMOOTH: u32                         = 0x1d01;
const GL_DEPTH_TEST: u32                     = 0x0b71;
const GL_LEQUAL: u32                         = 0x0203;

static XROT: f32 = 0.0;
static YROT: f32 = 0.0;
static ZROT: f32 = 0.0;

static FERRIS: u32 = 0;
static GCCLOGO: u32 = 0;
static CLAW: u32 = 0;

fn draw_gl() {
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glLoadIdentity();
        glTranslatef(0.0, 0.0, -5.0);

        glRotatef(XROT, 1.0, 0.0, 0.0);
        glRotatef(YROT, 0.0, 1.0, 0.0);
        glRotatef(ZROT, 0.0, 0.0, 1.0);

        glBindTexture(GL_TEXTURE_2D, CLAW);
        glBegin(GL_QUADS);

        // Front face
        glColor3f(1.0, 1.0, 1.0);
        glTexCoord2f(0.0, 0.0);
        glVertex3f(-1.0, -1.0,  1.0);
        glTexCoord2f(1.0, 0.0);
        glVertex3f(1.0, -1.0,  1.0);
        glTexCoord2f(1.0, 1.0);
        glVertex3f(1.0,  1.0,  1.0);
        glTexCoord2f(0.0, 1.0);
        glVertex3f(-1.0,  1.0,  1.0);

        // Back face
        glColor3f(1.0, 1.0, 1.0);
        glTexCoord2f(1.0, 0.0);
        glVertex3f(-1.0, -1.0, -1.0);
        glTexCoord2f(1.0, 1.0);
        glVertex3f(-1.0,  1.0, -1.0);
        glTexCoord2f(0.0, 1.0);
        glVertex3f(1.0,  1.0, -1.0);
        glTexCoord2f(0.0, 0.0);
        glVertex3f(1.0, -1.0, -1.0);

        glEnd();
        glBindTexture(GL_TEXTURE_2D, GCCLOGO);
        glBegin(GL_QUADS);

        // Top face
        glColor3f(1.0, 1.0, 1.0);
        glTexCoord2f(0.0, 1.0);
        glVertex3f(-1.0,  1.0, -1.0);
        glTexCoord2f(0.0, 0.0);
        glVertex3f(-1.0,  1.0,  1.0);
        glTexCoord2f(1.0, 0.0);
        glVertex3f(1.0,  1.0,  1.0);
        glTexCoord2f(1.0, 1.0);
        glVertex3f(1.0,  1.0, -1.0);


        // Bottom face
        glColor3f(1.0, 1.0, 1.0);
        glTexCoord2f(1.0, 1.0);
        glVertex3f(-1.0, -1.0, -1.0);
        glTexCoord2f(0.0, 1.0);
        glVertex3f(1.0, -1.0, -1.0);
        glTexCoord2f(0.0, 0.0);
        glVertex3f(1.0, -1.0,  1.0);
        glTexCoord2f(1.0, 0.0);
        glVertex3f(-1.0, -1.0,  1.0);

        glEnd();
        glBindTexture(GL_TEXTURE_2D, FERRIS);
        glBegin(GL_QUADS);

        // Right face
        glColor3f(0.0, 1.0, 0.0);
        glTexCoord2f(1.0, 0.0);
        glVertex3f(1.0, -1.0, -1.0);
        glColor3f(0.3, 0.5, 1.0);
        glTexCoord2f(1.0, 1.0);
        glVertex3f(1.0,  1.0, -1.0);
        glColor3f(1.0, 0.3, 0.5);
        glTexCoord2f(0.0, 1.0);
        glVertex3f(1.0,  1.0,  1.0);
        glColor3f(0.5, 0.5, 0.5);
        glTexCoord2f(0.0, 0.0);
        glVertex3f(1.0, -1.0,  1.0);

        // Left face
        glColor3f(1.0, 0.0, 0.0);
        glTexCoord2f(0.0, 0.0);
        glVertex3f(-1.0, -1.0, -1.0);
        glColor3f(1.0, 1.0, 0.0);
        glTexCoord2f(1.0, 0.0);
        glVertex3f(-1.0, -1.0,  1.0);
        glColor3f(0.0, 1.0, 1.0);
        glTexCoord2f(1.0, 1.0);
        glVertex3f(-1.0,  1.0,  1.0);
        glColor3f(0.0, 0.0, 1.0);
        glTexCoord2f(0.0, 1.0);
        glVertex3f(-1.0,  1.0, -1.0);

        glEnd();

        XROT += 0.8;
        YROT += 0.6;
        ZROT += 1.0;
    }
}

#[no_mangle]
pub extern fn main() -> i32 {
    unsafe {
        // Initialize GLdc
        glKosInit();

        // Say hello to the world!
        printf("\nWelcome to Rust on Sega Dreamcast!\n");

        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        gluPerspective(45.0, 640.0 / 480.0, 0.1, 100.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        glEnable(GL_TEXTURE_2D);
        glShadeModel(GL_SMOOTH);
        glClearColor(0.0, 0.0, 0.0, 0.5);
        glClearDepth(1.0);
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LEQUAL);

        // Set up textures
        glGenTextures(1, &FERRIS);
        glBindTexture(GL_TEXTURE_2D, FERRIS);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, data_size(&ferris, &ferris_end), &ferris);

        glGenTextures(1, &GCCLOGO);
        glBindTexture(GL_TEXTURE_2D, GCCLOGO);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, data_size(&gcc, &gcc_end), &gcc);

        glGenTextures(1, &CLAW);
        glBindTexture(GL_TEXTURE_2D, CLAW);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, data_size(&claw, &claw_end), &claw);

        loop {
            // Exit if there are no controllers
            let mapledev = maple_enum_type(0, MAPLE_FUNC_CONTROLLER);

            if is_null(mapledev as *mut u8) {
                printf("No controllers connected! Quitting...\n");
                break;
            }

            // Exit if input cannot be read
            let state: *mut cont_state = maple_dev_status(mapledev);

            if is_null(state as *mut u8) {
                printf("Error reading controller! Quitting...\n");
                break;
            }

            // Exit when the user presses start
            if ((*state).buttons >> 3) == 1 {
                printf("Start pressed. Goodbye!\n");
                break;
            }

            draw_gl();

            glKosSwapBuffers();
        }

        glDeleteTextures(1, &FERRIS);
        glDeleteTextures(2, &GCCLOGO);
        glDeleteTextures(3, &CLAW);
    }

    return 0;
}
