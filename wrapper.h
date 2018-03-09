#include <xwiimote.h>

/* interfaces */
enum xwii_if_base_idx {
	/* base interfaces */
	XWII_IF_CORE,
	XWII_IF_ACCEL,
	XWII_IF_IR,

	/* extension interfaces */
	XWII_IF_MOTION_PLUS,
	XWII_IF_NUNCHUK,
	XWII_IF_CLASSIC_CONTROLLER,
	XWII_IF_BALANCE_BOARD,
	XWII_IF_PRO_CONTROLLER,
	XWII_IF_DRUMS,
	XWII_IF_GUITAR,

	XWII_IF_NUM,
};

/* event interface */
struct xwii_if {
	/* device node as /dev/input/eventX or NULL */
	char *node;
	/* open file or -1 */
	int fd;
	/* temporary state during device detection */
	unsigned int available : 1;
};

struct xwii_iface {
	/* reference count */
	size_t ref;
	/* epoll file descriptor */
	int efd;
	/* udev context */
	struct udev *udev;
	/* main udev device */
	struct udev_device *dev;
	/* udev monitor */
	struct udev_monitor *umon;

	/* bitmask of open interfaces */
	unsigned int ifaces;
	/* interfaces */
	struct xwii_if ifs[XWII_IF_NUM];
	/* device type attribute */
	char *devtype_attr;
	/* extension attribute */
	char *extension_attr;
	/* battery capacity attribute */
	char *battery_attr;
	/* led brightness attributes */
	char *led_attrs[4];

	/* rumble-id for base-core interface force-feedback or -1 */
	int rumble_id;
	int rumble_fd;
	/* accelerometer data cache */
	struct xwii_event_abs accel_cache;
	/* IR data cache */
	struct xwii_event_abs ir_cache[4];
	/* balance board weight cache */
	struct xwii_event_abs bboard_cache[4];
	/* motion plus cache */
	struct xwii_event_abs mp_cache;
	/* motion plus normalization */
	struct xwii_event_abs mp_normalizer;
	int32_t mp_normalize_factor;
	/* pro controller cache */
	struct xwii_event_abs pro_cache[2];
	/* classic controller cache */
	struct xwii_event_abs classic_cache[3];
	/* nunchuk cache */
	struct xwii_event_abs nunchuk_cache[2];
	/* drums cache */
	struct xwii_event_abs drums_cache[XWII_DRUMS_ABS_NUM];
	/* guitar cache */
	struct xwii_event_abs guitar_cache[3];
};
