#include "install.h"
#include "fs/bobbyfs/bobbyfs.h"
#include <debug.h>
#include <fs.h>

void install()
{
    printd("Installing...\n");

    printd("Formatting fs... ");
    bobbyfs_format();
    printd("OK.\n");

    bobbyfs_dir root = bobbyfs_dir_root();

    

    bobbyfs_dir_close(root);
    printd("Installation complete.\n");
}
