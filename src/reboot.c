#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/reboot.h>
#include <unistd.h>

// https://man7.org/linux/man-pages/man2/reboot.2.html

int sys_reboot() { return reboot(RB_AUTOBOOT); }

int sys_poweroff() { return reboot(RB_POWER_OFF); }

int sys_suspend() {
  // Alternative that works but get instantly woken up, requires root
	// permissions.
  // system("echo -n mem > /sys/power/state");

  char *newargv[] = {"/usr/bin/systemctl", "suspend", NULL};
  char *newenviron[] = {NULL};
  execve(newargv[0], newargv, newenviron);
  return 0;

  // Hibernate call is fine but it freezes and requires more configuration.
  // return reboot(RB_SW_SUSPEND);
}

int sys_logout() {
  // system("kill -9 -1");
  system("loginctl terminate-user $USER");
  return 0;
}

int sys_lock(const char *cmd) {
  if (strlen(cmd) != 0) {
    system(cmd);
    return 0;
  }
  system("loginctl lock-session");
  return 0;
}
