extern crate gcc;
extern crate pkg_config;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};


fn build_libz(libz_sources: &[&str]) {
    let mut config = gcc::Build::new();
    for c_file in libz_sources {
        config.file(c_file);
    }
    config.flag("-w");
    config.compile("libz.a");
}

fn build_libtcod_objects(mut config: gcc::Build, sources: &[&str]) {
    config.include("libtcod/include");
    config.include("libtcod/src/zlib");
    for c_file in sources {
        config.file(c_file);
    }
    config.cargo_metadata(false);
    config.flag("-w");
    config.compile("libtcod.a");
}


fn compile_config(config: gcc::Build) {
    let mut cmd = config.get_compiler().to_command();
    println!("Compiling: {:?}", cmd);
    match cmd.output() {
        Ok(output) => {
            println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            if !output.status.success() {
                panic!("Compilation failed.");
            }
        }
        Err(e) => {
            panic!("Failed to run the compilation command {}.", e);
        }
    }
}


fn main() {
    let is_crater = option_env!("CRATER_TASK_TYPE");

    if is_crater.is_some() {
        return;
    }

    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    let src = Path::new(&src_dir);
    let dst = Path::new(&dst_dir);
    let sdl_lib_dir = src.join("libtcod/dependencies/SDL2-2.0.5/lib").join(&target);
    let sdl_include_dir = src.join("libtcod/dependencies/SDL2-2.0.5/include").join(&target);

    let libz_sources = &[
        "libtcod/src/zlib/adler32.c",
	    "libtcod/src/zlib/crc32.c",
	    "libtcod/src/zlib/deflate.c",
	    "libtcod/src/zlib/infback.c",
	    "libtcod/src/zlib/inffast.c",
	    "libtcod/src/zlib/inflate.c",
	    "libtcod/src/zlib/inftrees.c",
	    "libtcod/src/zlib/trees.c",
	    "libtcod/src/zlib/zutil.c",
	    "libtcod/src/zlib/compress.c",
	    "libtcod/src/zlib/uncompr.c",
	    "libtcod/src/zlib/gzclose.c",
	    "libtcod/src/zlib/gzlib.c",
	    "libtcod/src/zlib/gzread.c",
	    "libtcod/src/zlib/gzwrite.c",
    ];

    let libtcod_sources = &[
 	    "libtcod/src/bresenham_c.c",
	    "libtcod/src/bsp_c.c",
	    "libtcod/src/color_c.c",
	    "libtcod/src/console_c.c",
        "libtcod/src/console_rexpaint.c",
	    "libtcod/src/fov_c.c",
	    "libtcod/src/fov_circular_raycasting.c",
	    "libtcod/src/fov_diamond_raycasting.c",
	    "libtcod/src/fov_permissive2.c",
	    "libtcod/src/fov_recursive_shadowcasting.c",
	    "libtcod/src/fov_restrictive.c",
	    "libtcod/src/heightmap_c.c",
	    "libtcod/src/image_c.c",
	    "libtcod/src/lex_c.c",
	    "libtcod/src/list_c.c",
	    "libtcod/src/mersenne_c.c",
	    "libtcod/src/namegen_c.c",
	    "libtcod/src/noise_c.c",
	    "libtcod/src/parser_c.c",
	    "libtcod/src/path_c.c",
	    "libtcod/src/sys_c.c",
	    "libtcod/src/sys_sdl2_c.c",
	    "libtcod/src/sys_sdl_c.c",
	    "libtcod/src/sys_sdl_img_bmp.c",
	    "libtcod/src/sys_sdl_img_png.c",
	    "libtcod/src/tree_c.c",
	    "libtcod/src/txtfield_c.c",
	    "libtcod/src/wrappers.c",
	    "libtcod/src/zip_c.c",
	    "libtcod/src/png/lodepng.c",
        /*
        "libtcod/src/gui/button.cpp",
        "libtcod/src/gui/container.cpp",
        "libtcod/src/gui/flatlist.cpp",
        "libtcod/src/gui/hbox.cpp",
        "libtcod/src/gui/image.cpp",
        "libtcod/src/gui/label.cpp",
        "libtcod/src/gui/radiobutton.cpp",
        "libtcod/src/gui/slider.cpp",
        "libtcod/src/gui/statusbar.cpp",
        "libtcod/src/gui/textbox.cpp",
        "libtcod/src/gui/togglebutton.cpp",
        "libtcod/src/gui/toolbar.cpp",
        "libtcod/src/gui/vbox.cpp",
        "libtcod/src/gui/widget.cpp",
        */
    ];

    if target.contains("linux") {
        build_libz(libz_sources);

        // Build the *.o files:
        {
            let mut config = gcc::Build::new();
            for include_path in &pkg_config::find_library("sdl2").unwrap().include_paths {
                config.include(include_path);
            }
            config.define("TCOD_SDL2", None);
            config.define("NO_OPENGL", None);
            config.flag("-fno-strict-aliasing");
            config.flag("-ansi");
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Build::new();
        config.define("TCOD_SDL2", None);
        config.define("NO_OPENGL", None);
        config.flag("-shared");
        config.flag("-Wl,-soname,libtcod.so");
        config.flag("-o");
        config.flag(dst.join("libtcod.so").to_str().unwrap());
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag("-lSDL2");
        config.flag("-lGL");
        config.flag("-lX11");
        config.flag("-lm");
        config.flag("-ldl");
        config.flag("-lpthread");

        compile_config(config);
        assert!(dst.join("libtcod.so").is_file());

        pkg_config::find_library("gl").unwrap();
        pkg_config::find_library("x11").unwrap();

    } else if target.contains("darwin") {
        build_libz(libz_sources);

        // Build the *.o files
        {
            let mut config = gcc::Build::new();
            for include_path in &pkg_config::find_library("sdl2").unwrap().include_paths {
                config.include(include_path);
            }
            config.define("TCOD_SDL2", None);
            config.define("NO_OPENGL", None);
            config.flag("-fno-strict-aliasing");
            config.flag("-ansi");
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Build::new();
        config.define("TCOD_SDL2", None);
        config.define("NO_OPENGL", None);
        config.flag("-shared");
        config.flag("-o");
        config.flag(dst.join("libtcod.dylib").to_str().unwrap());
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag(src.join("libtcod/osx/macsupport.m").to_str().unwrap());
        config.flag("-lSDL2");
        config.flag("-lSDL2main");
        config.flag("-framework");
        config.flag("OpenGL");
        config.flag("-framework");
        config.flag("Cocoa");
        config.flag("-lm");
        config.flag("-ldl");
        config.flag("-lpthread");

        compile_config(config);
        assert!(dst.join("libtcod.dylib").is_file());

        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");

    } else if target.contains("windows-gnu") {
        assert!(sdl_lib_dir.is_dir());
        assert!(sdl_include_dir.is_dir());
        fs::copy(&sdl_lib_dir.join("SDL2.dll"), &dst.join("SDL2.dll")).unwrap();

        build_libz(libz_sources);

        // Build the *.o files:
        {
            let mut config = gcc::Build::new();
            config.include(sdl_include_dir.to_str().unwrap());
            config.flag("-fno-strict-aliasing");
            config.flag("-ansi");
            config.define("TCOD_SDL2", None);
            config.define("NO_OPENGL", None);
            config.define("LIBTCOD_EXPORTS", None);
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Build::new();
        config.define("TCOD_SDL2", None);
        config.define("NO_OPENGL", None);
        config.flag("-o");
        config.flag(dst.join("libtcod.dll").to_str().unwrap());
        config.flag("-shared");
        fs::create_dir(dst.join("lib")).unwrap();
        config.flag(&format!("-Wl,--out-implib,{}", dst.join("lib/libtcod.a").display()));
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag("-mwindows");
        config.flag("-L");
        config.flag(sdl_lib_dir.to_str().unwrap());
        config.flag("-lSDL2.dll");
        config.flag("-lopengl32");
        config.flag("-static-libgcc");
        config.flag("-static-libstdc++");

        compile_config(config);
        assert!(dst.join("libtcod.dll").is_file());

        println!("cargo:rustc-link-lib=dylib={}", "SDL2");
        println!("cargo:rustc-link-lib=dylib={}", "opengl32");
        println!("cargo:rustc-link-search=native={}", sdl_lib_dir.display());
        println!("cargo:rustc-link-search=native={}", dst.display());

    } else if target.contains("windows-msvc") {
        assert!(sdl_lib_dir.is_dir());
        assert!(sdl_include_dir.is_dir());
        fs::copy(&sdl_lib_dir.join("SDL2.dll"), &dst.join("SDL2.dll")).unwrap();
        fs::copy(&sdl_lib_dir.join("SDL2.lib"), &dst.join("SDL2.lib")).unwrap();
        fs::copy(&sdl_lib_dir.join("SDL2main.lib"), &dst.join("SDL2main.lib")).unwrap();

        // Build the DLL
        let mut config = gcc::Build::new();
        config.define("TCOD_SDL2", None);
        config.define("NO_OPENGL", None);
        config.flag("/DLIBTCOD_EXPORTS");
        config.flag("/DNO_OPENGL");
        config.include(sdl_include_dir.to_str().unwrap());
        config.include(Path::new("libtcod").join("src").join("zlib"));
        config.include(Path::new("libtcod").join("include"));
        for c_file in libz_sources.iter().chain(libtcod_sources) {
            // Make sure the path is in the Windows format. This
            // shouldn't matter but it's distracting when debugging
            // build script issues.
            let path = c_file.split('/').fold(PathBuf::new(), |path, segment| path.join(segment));
            config.flag(src.join(path).to_str().unwrap());
        }
        config.flag("User32.lib");
        config.flag("SDL2.lib");
        config.flag("SDL2main.lib");
        config.flag("/link");
        config.flag(&format!("/LIBPATH:{}", dst.to_str().unwrap()));
        config.flag("/DLL");
        config.flag(&format!("/OUT:{}", dst.join("tcod.dll").display()));

        compile_config(config);
        assert!(dst.join("tcod.dll").is_file());

        println!("cargo:rustc-link-search={}", dst.display());
        println!("cargo:rustc-link-lib=dylib=SDL2");
        println!("cargo:rustc-link-lib=User32");
    }

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=tcod");
}
