#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/reboot.h>
#include <unistd.h>

// https://man7.org/linux/man-pages/man2/reboot.2.html

// System reboot
int sys_reboot() { return reboot(RB_AUTOBOOT); }

int sys_poweroff() { return reboot(RB_POWER_OFF); }

int sys_hibernate() { return reboot(RB_SW_SUSPEND); }

int sys_logout() {
  // system("kill -9 -1");
  system("loginctl terminate-user $USER");
  return 0;
}

int sys_lock(const char* cmd) {
  if (strlen(cmd) != 0) {
    system(cmd);
    return 0;
  }
  system("loginctl lock-session");
  return 0;
}
