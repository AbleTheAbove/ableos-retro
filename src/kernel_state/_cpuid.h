/*===---- cpuid.h - X86 cpu model detection --------------------------------===
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 *===-----------------------------------------------------------------------===
 */

#if !(__x86_64__ || __i386__)
#error this header is for x86 only
#endif




#if __i386__
#define __cpuid(__leaf, __eax, __ebx, __ecx, __edx) \
    __asm("cpuid" : "=a"(__eax), "=b" (__ebx), "=c"(__ecx), "=d"(__edx) \
                  : "0"(__leaf))

#define __cpuid_count(__leaf, __count, __eax, __ebx, __ecx, __edx) \
    __asm("cpuid" : "=a"(__eax), "=b" (__ebx), "=c"(__ecx), "=d"(__edx) \
                  : "0"(__leaf), "2"(__count))
#else
/* x86-64 uses %rbx as the base register, so preserve it. */
#define __cpuid(__leaf, __eax, __ebx, __ecx, __edx) \
    __asm("  xchgq  %%rbx,%q1\n" \
          "  cpuid\n" \
          "  xchgq  %%rbx,%q1" \
        : "=a"(__eax), "=r" (__ebx), "=c"(__ecx), "=d"(__edx) \
        : "0"(__leaf))

#define __cpuid_count(__leaf, __count, __eax, __ebx, __ecx, __edx) \
    __asm("  xchgq  %%rbx,%q1\n" \
          "  cpuid\n" \
          "  xchgq  %%rbx,%q1" \
        : "=a"(__eax), "=r" (__ebx), "=c"(__ecx), "=d"(__edx) \
        : "0"(__leaf), "2"(__count))
#endif

static __inline int __get_cpuid_max (unsigned int __leaf, unsigned int *__sig)
{
    unsigned int __eax, __ebx, __ecx, __edx;
#if __i386__
    int __cpuid_supported;

    __asm("  pushfl\n"
          "  popl   %%eax\n"
          "  movl   %%eax,%%ecx\n"
          "  xorl   $0x00200000,%%eax\n"
          "  pushl  %%eax\n"
          "  popfl\n"
          "  pushfl\n"
          "  popl   %%eax\n"
          "  movl   $0,%0\n"
          "  cmpl   %%eax,%%ecx\n"
          "  je     1f\n"
          "  movl   $1,%0\n"
          "1:"
        : "=r" (__cpuid_supported) : : "eax", "ecx");
    if (!__cpuid_supported)
        return 0;
#endif

    __cpuid(__leaf, __eax, __ebx, __ecx, __edx);
    if (__sig)
        *__sig = __ebx;
    return __eax;
}

static __inline int __get_cpuid (unsigned int __leaf, unsigned int *__eax,
                                 unsigned int *__ebx, unsigned int *__ecx,
                                 unsigned int *__edx)
{
    unsigned int __max_leaf = __get_cpuid_max(__leaf & 0x80000000, 0);

    if (__max_leaf == 0 || __max_leaf < __leaf)
        return 0;

    __cpuid(__leaf, *__eax, *__ebx, *__ecx, *__edx);
    return 1;
}

static __inline int __get_cpuid_count (unsigned int __leaf,
                                       unsigned int __subleaf,
                                       unsigned int *__eax, unsigned int *__ebx,
                                       unsigned int *__ecx, unsigned int *__edx)
{
    unsigned int __max_leaf = __get_cpuid_max(__leaf & 0x80000000, 0);

    if (__max_leaf == 0 || __max_leaf < __leaf)
        return 0;

    __cpuid_count(__leaf, __subleaf, *__eax, *__ebx, *__ecx, *__edx);
    return 1;
}