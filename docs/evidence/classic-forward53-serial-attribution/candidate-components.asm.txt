Disassembly of section .text:

00000000000d0150 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}>:
   d0150:	55                                              	push   %rbp
   d0151:	41 57                                           	push   %r15
   d0153:	41 56                                           	push   %r14
   d0155:	41 55                                           	push   %r13
   d0157:	41 54                                           	push   %r12
   d0159:	53                                              	push   %rbx
   d015a:	48 83 ec 58                                     	sub    $0x58,%rsp
   d015e:	48 89 7c 24 08                                  	mov    %rdi,0x8(%rsp)
   d0163:	48 8b 0e                                        	mov    (%rsi),%rcx
   d0166:	48 8b 41 10                                     	mov    0x10(%rcx),%rax
   d016a:	48 85 c0                                        	test   %rax,%rax
   d016d:	0f 84 c4 00 00 00                               	je     d0237 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xe7>
   d0173:	4c 8b 69 08                                     	mov    0x8(%rcx),%r13
   d0177:	48 c1 e0 03                                     	shl    $0x3,%rax
   d017b:	48 8d 2c 40                                     	lea    (%rax,%rax,2),%rbp
   d017f:	4c 8b 76 08                                     	mov    0x8(%rsi),%r14
   d0183:	48 8b 46 10                                     	mov    0x10(%rsi),%rax
   d0187:	48 89 44 24 20                                  	mov    %rax,0x20(%rsp)
   d018c:	4d 8d be b0 00 00 00                            	lea    0xb0(%r14),%r15
   d0193:	48 8b 46 18                                     	mov    0x18(%rsi),%rax
   d0197:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   d019c:	48 8b 46 20                                     	mov    0x20(%rsi),%rax
   d01a0:	48 89 44 24 10                                  	mov    %rax,0x10(%rsp)
   d01a5:	31 db                                           	xor    %ebx,%ebx
   d01a7:	4c 8d 64 24 28                                  	lea    0x28(%rsp),%r12
   d01ac:	eb 4f                                           	jmp    d01fd <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xad>
   d01ae:	66 90                                           	xchg   %ax,%ax
   d01b0:	48 8b 44 24 20                                  	mov    0x20(%rsp),%rax
   d01b5:	8b 30                                           	mov    (%rax),%esi
   d01b7:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   d01bc:	8b 10                                           	mov    (%rax),%edx
   d01be:	49 8b 4c 1d 08                                  	mov    0x8(%r13,%rbx,1),%rcx
   d01c3:	4d 8b 44 1d 10                                  	mov    0x10(%r13,%rbx,1),%r8
   d01c8:	48 83 ec 08                                     	sub    $0x8,%rsp
   d01cc:	4c 89 e7                                        	mov    %r12,%rdi
   d01cf:	41 b9 02 00 00 00                               	mov    $0x2,%r9d
   d01d5:	ff 74 24 18                                     	push   0x18(%rsp)
   d01d9:	6a 1c                                           	push   $0x1c
   d01db:	48 8d 05 3b 3b f4 ff                            	lea    -0xbc4c5(%rip),%rax        # 13d1d <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xfbd>
   d01e2:	50                                              	push   %rax
   d01e3:	e8 b8 58 04 00                                  	call   115aa0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch>
   d01e8:	48 83 c4 20                                     	add    $0x20,%rsp
   d01ec:	48 83 7c 24 28 ff                               	cmpq   $0xffffffffffffffff,0x28(%rsp)
   d01f2:	75 5e                                           	jne    d0252 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0x102>
   d01f4:	48 83 c3 18                                     	add    $0x18,%rbx
   d01f8:	48 39 dd                                        	cmp    %rbx,%rbp
   d01fb:	74 3a                                           	je     d0237 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xe7>
   d01fd:	49 83 3f ff                                     	cmpq   $0xffffffffffffffff,(%r15)
   d0201:	74 ad                                           	je     d01b0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0x60>
   d0203:	49 8b 74 1d 08                                  	mov    0x8(%r13,%rbx,1),%rsi
   d0208:	49 8b 54 1d 10                                  	mov    0x10(%r13,%rbx,1),%rdx
   d020d:	4c 89 e7                                        	mov    %r12,%rdi
   d0210:	4c 89 f1                                        	mov    %r14,%rcx
   d0213:	4d 89 f8                                        	mov    %r15,%r8
   d0216:	ff 15 9c 51 1a 00                               	call   *0x1a519c(%rip)        # 2753b8 <_DYNAMIC+0x808>
   d021c:	83 7c 24 28 ff                                  	cmpl   $0xffffffff,0x28(%rsp)
   d0221:	74 d1                                           	je     d01f4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xa4>
   d0223:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d022d:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d0232:	48 89 01                                        	mov    %rax,(%rcx)
   d0235:	eb 0c                                           	jmp    d0243 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xf3>
   d0237:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
   d023c:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
   d0243:	48 83 c4 58                                     	add    $0x58,%rsp
   d0247:	5b                                              	pop    %rbx
   d0248:	41 5c                                           	pop    %r12
   d024a:	41 5d                                           	pop    %r13
   d024c:	41 5e                                           	pop    %r14
   d024e:	41 5f                                           	pop    %r15
   d0250:	5d                                              	pop    %rbp
   d0251:	c3                                              	ret
   d0252:	0f 10 44 24 28                                  	movups 0x28(%rsp),%xmm0
   d0257:	0f 10 4c 24 38                                  	movups 0x38(%rsp),%xmm1
   d025c:	0f 10 54 24 48                                  	movups 0x48(%rsp),%xmm2
   d0261:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
   d0266:	0f 11 50 20                                     	movups %xmm2,0x20(%rax)
   d026a:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
   d026e:	0f 11 00                                        	movups %xmm0,(%rax)
   d0271:	eb d0                                           	jmp    d0243 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xf3>
