Disassembly of section .text:

00000000000c3770 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}>:
   c3770:	55                                              	push   %rbp
   c3771:	41 57                                           	push   %r15
   c3773:	41 56                                           	push   %r14
   c3775:	41 55                                           	push   %r13
   c3777:	41 54                                           	push   %r12
   c3779:	53                                              	push   %rbx
   c377a:	48 83 ec 38                                     	sub    $0x38,%rsp
   c377e:	48 89 3c 24                                     	mov    %rdi,(%rsp)
   c3782:	48 8b 0e                                        	mov    (%rsi),%rcx
   c3785:	48 8b 41 10                                     	mov    0x10(%rcx),%rax
   c3789:	48 85 c0                                        	test   %rax,%rax
   c378c:	74 67                                           	je     c37f5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0x85>
   c378e:	4c 8b 79 08                                     	mov    0x8(%rcx),%r15
   c3792:	48 c1 e0 03                                     	shl    $0x3,%rax
   c3796:	4c 8d 24 40                                     	lea    (%rax,%rax,2),%r12
   c379a:	4c 8b 6e 08                                     	mov    0x8(%rsi),%r13
   c379e:	48 8b 6e 10                                     	mov    0x10(%rsi),%rbp
   c37a2:	48 8b 5e 18                                     	mov    0x18(%rsi),%rbx
   c37a6:	45 31 f6                                        	xor    %r14d,%r14d
   c37a9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
   c37b0:	41 8b 75 00                                     	mov    0x0(%r13),%esi
   c37b4:	8b 55 00                                        	mov    0x0(%rbp),%edx
   c37b7:	4b 8b 4c 37 08                                  	mov    0x8(%r15,%r14,1),%rcx
   c37bc:	4f 8b 44 37 10                                  	mov    0x10(%r15,%r14,1),%r8
   c37c1:	48 83 ec 08                                     	sub    $0x8,%rsp
   c37c5:	48 8d 7c 24 10                                  	lea    0x10(%rsp),%rdi
   c37ca:	41 b9 02 00 00 00                               	mov    $0x2,%r9d
   c37d0:	53                                              	push   %rbx
   c37d1:	6a 1c                                           	push   $0x1c
   c37d3:	48 8d 05 23 01 f5 ff                            	lea    -0xafedd(%rip),%rax        # 138fd <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xfbd>
   c37da:	50                                              	push   %rax
   c37db:	e8 50 10 05 00                                  	call   114830 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch>
   c37e0:	48 83 c4 20                                     	add    $0x20,%rsp
   c37e4:	48 83 7c 24 08 ff                               	cmpq   $0xffffffffffffffff,0x8(%rsp)
   c37ea:	75 16                                           	jne    c3802 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0x92>
   c37ec:	49 83 c6 18                                     	add    $0x18,%r14
   c37f0:	4d 39 f4                                        	cmp    %r14,%r12
   c37f3:	75 bb                                           	jne    c37b0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0x40>
   c37f5:	48 8b 04 24                                     	mov    (%rsp),%rax
   c37f9:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
   c3800:	eb 1e                                           	jmp    c3820 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}+0xb0>
   c3802:	0f 10 44 24 08                                  	movups 0x8(%rsp),%xmm0
   c3807:	0f 10 4c 24 18                                  	movups 0x18(%rsp),%xmm1
   c380c:	0f 10 54 24 28                                  	movups 0x28(%rsp),%xmm2
   c3811:	48 8b 04 24                                     	mov    (%rsp),%rax
   c3815:	0f 11 50 20                                     	movups %xmm2,0x20(%rax)
   c3819:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
   c381d:	0f 11 00                                        	movups %xmm0,(%rax)
   c3820:	48 83 c4 38                                     	add    $0x38,%rsp
   c3824:	5b                                              	pop    %rbx
   c3825:	41 5c                                           	pop    %r12
   c3827:	41 5d                                           	pop    %r13
   c3829:	41 5e                                           	pop    %r14
   c382b:	41 5f                                           	pop    %r15
   c382d:	5d                                              	pop    %rbp
   c382e:	c3                                              	ret
