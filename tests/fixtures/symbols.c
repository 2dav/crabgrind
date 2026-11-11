typedef unsigned char uint8_t;
typedef unsigned int uint32_t;

_Static_assert(__CHAR_BIT__ == 8, "8-bit bytes required");

#define SYM __attribute__((used))
#define ALIGN(n) __attribute__((aligned(n)))
#ifdef __APPLE__
#define SEC(name) __attribute__((section("__DATA," name)))
#else
#define SEC(name) __attribute__((section(name)))
#endif

SYM const uint32_t CG_TEST_ZERO = 0x00000000;
SYM const uint32_t CG_TEST_MAX = 0xFFFFFFFF;

SYM const uint32_t CG_TEST_A SEC(".cg_pad") = 0x12345678;
SYM const uint8_t CG_TEST_PADDING[64] SEC(".cg_pad") = {0};
SYM const uint32_t CG_TEST_B SEC(".cg_pad") = 0xAABBCCDD;

SYM const uint32_t CG_TEST_SECTION_A SEC(".cg_test_a") = 0x11111111;
SYM const uint32_t CG_TEST_ALIGN_16 SEC(".cg_test_a") ALIGN(16) = 0x33333333;
SYM const uint32_t CG_TEST_SECTION_B SEC(".cg_test_b") = 0x22222222;
SYM const uint32_t CG_TEST_ALIGN_32 SEC(".cg_test_b") ALIGN(32) = 0x44444444;

#if defined(__ELF__) && defined(__arm__)
__asm__(".pushsection .cg_test_zero, \"a\", %progbits\n"
        ".globl CG_TEST_ZERO_SIZE\n"
        ".type CG_TEST_ZERO_SIZE, %object\n"
        "CG_TEST_ZERO_SIZE:\n"
        ".size CG_TEST_ZERO_SIZE, 0\n"
        ".popsection\n");
#elif defined(__ELF__)
__asm__(".pushsection .cg_test_zero, \"a\", @progbits\n"
        ".globl CG_TEST_ZERO_SIZE\n"
        ".type CG_TEST_ZERO_SIZE, @object\n"
        "CG_TEST_ZERO_SIZE:\n"
        ".size CG_TEST_ZERO_SIZE, 0\n"
        ".popsection\n");
#elif defined(__APPLE__) && defined(__MACH__)
__asm__(".section __DATA,.cg_test_zero\n"
        ".globl _CG_TEST_ZERO_SIZE\n"
        "_CG_TEST_ZERO_SIZE:\n");
#endif

SYM const uint32_t CG_TEST_HI = 0x80000000;
SYM const uint32_t CG_TEST_LO = 0x000000FF;
SYM const uint32_t CG_TEST_MI = 0xBEDABEDA;

SYM const uint32_t CG_TEST_DUP = 0xBEDABEDA;
