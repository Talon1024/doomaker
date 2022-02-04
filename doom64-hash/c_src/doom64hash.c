#include <ctype.h>
#include <stdint.h>

uint16_t hash(const char* str) {
	uint32_t hash_ = 1315423911;
	for (int i = 0; i < 8 && str[i]; ++i) {
		char c = str[i];
		hash_ ^= (hash_ << 5) + toupper(c) + (hash_ >> 2);
	}
	return (uint16_t)(hash_ & 0xffff);
}
