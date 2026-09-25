Disassembly of section .text:

0000000000150330 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2>:
  150330:	55                                              	push   %rbp
  150331:	41 57                                           	push   %r15
  150333:	41 56                                           	push   %r14
  150335:	41 55                                           	push   %r13
  150337:	41 54                                           	push   %r12
  150339:	53                                              	push   %rbx
  15033a:	48 81 ec 18 03 00 00                            	sub    $0x318,%rsp
  150341:	49 89 ff                                        	mov    %rdi,%r15
  150344:	89 74 24 04                                     	mov    %esi,0x4(%rsp)
  150348:	89 54 24 24                                     	mov    %edx,0x24(%rsp)
  15034c:	49 81 f9 ff ff 00 00                            	cmp    $0xffff,%r9
  150353:	76 12                                           	jbe    150367 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x37>
  150355:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  15035f:	49 89 07                                        	mov    %rax,(%r15)
  150362:	e9 b3 01 00 00                                  	jmp    15051a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1ea>
  150367:	4c 89 cb                                        	mov    %r9,%rbx
  15036a:	41 89 cc                                        	mov    %ecx,%r12d
  15036d:	4d 89 c6                                        	mov    %r8,%r14
  150370:	4c 8b 8c 24 58 03 00 00                         	mov    0x358(%rsp),%r9
  150378:	4c 8b 84 24 50 03 00 00                         	mov    0x350(%rsp),%r8
  150380:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150388:	89 d9                                           	mov    %ebx,%ecx
  15038a:	e8 11 98 f4 ff                                  	call   99ba0 <emuella_j2k_codestream::scalable_lossless::execution_requirements::<false>>
  15038f:	48 8b 84 24 50 01 00 00                         	mov    0x150(%rsp),%rax
  150397:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  15039b:	74 32                                           	je     1503cf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x9f>
  15039d:	48 8b 8c 24 78 01 00 00                         	mov    0x178(%rsp),%rcx
  1503a5:	f3 0f 6f 84 24 58 01 00 00                      	movdqu 0x158(%rsp),%xmm0
  1503ae:	f3 0f 6f 8c 24 68 01 00 00                      	movdqu 0x168(%rsp),%xmm1
  1503b7:	49 89 07                                        	mov    %rax,(%r15)
  1503ba:	f3 41 0f 7f 47 08                               	movdqu %xmm0,0x8(%r15)
  1503c0:	f3 41 0f 7f 4f 18                               	movdqu %xmm1,0x18(%r15)
  1503c6:	49 89 4f 28                                     	mov    %rcx,0x28(%r15)
  1503ca:	e9 4b 01 00 00                                  	jmp    15051a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1ea>
  1503cf:	48 8b 94 24 78 01 00 00                         	mov    0x178(%rsp),%rdx
  1503d7:	41 0f b6 f4                                     	movzbl %r12b,%esi
  1503db:	40 80 fe 10                                     	cmp    $0x10,%sil
  1503df:	74 09                                           	je     1503ea <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xba>
  1503e1:	83 fe 08                                        	cmp    $0x8,%esi
  1503e4:	0f 85 f9 00 00 00                               	jne    1504e3 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1b3>
  1503ea:	48 83 fb 08                                     	cmp    $0x8,%rbx
  1503ee:	0f 94 c0                                        	sete   %al
  1503f1:	41 80 fc 10                                     	cmp    $0x10,%r12b
  1503f5:	0f 95 c1                                        	setne  %cl
  1503f8:	84 c1                                           	test   %al,%cl
  1503fa:	0f 85 e3 00 00 00                               	jne    1504e3 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1b3>
  150400:	48 89 b4 24 40 02 00 00                         	mov    %rsi,0x240(%rsp)
  150408:	48 89 94 24 48 01 00 00                         	mov    %rdx,0x148(%rsp)
  150410:	44 89 e0                                        	mov    %r12d,%eax
  150413:	c0 e8 03                                        	shr    $0x3,%al
  150416:	44 0f b6 c0                                     	movzbl %al,%r8d
  15041a:	8b 44 24 04                                     	mov    0x4(%rsp),%eax
  15041e:	8b 54 24 24                                     	mov    0x24(%rsp),%edx
  150422:	49 89 d5                                        	mov    %rdx,%r13
  150425:	4c 0f af e8                                     	imul   %rax,%r13
  150429:	48 89 9c 24 c8 00 00 00                         	mov    %rbx,0xc8(%rsp)
  150431:	49 89 d9                                        	mov    %rbx,%r9
  150434:	49 c1 e1 05                                     	shl    $0x5,%r9
  150438:	4d 89 f2                                        	mov    %r14,%r10
  15043b:	4b 8d 0c 0e                                     	lea    (%r14,%r9,1),%rcx
  15043f:	48 89 8c 24 e0 00 00 00                         	mov    %rcx,0xe0(%rsp)
  150447:	48 89 84 24 a8 00 00 00                         	mov    %rax,0xa8(%rsp)
  15044f:	48 8d 48 ff                                     	lea    -0x1(%rax),%rcx
  150453:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
  150458:	48 8d 72 ff                                     	lea    -0x1(%rdx),%rsi
  15045c:	0f 1f 40 00                                     	nopl   0x0(%rax)
  150460:	4d 85 c9                                        	test   %r9,%r9
  150463:	0f 84 c6 00 00 00                               	je     15052f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1ff>
  150469:	4d 8b 5a 18                                     	mov    0x18(%r10),%r11
  15046d:	48 89 c8                                        	mov    %rcx,%rax
  150470:	49 f7 e3                                        	mul    %r11
  150473:	0f 80 dc fe ff ff                               	jo     150355 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  150479:	48 89 c7                                        	mov    %rax,%rdi
  15047c:	4c 01 c7                                        	add    %r8,%rdi
  15047f:	0f 82 d0 fe ff ff                               	jb     150355 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  150485:	49 8b 5a 10                                     	mov    0x10(%r10),%rbx
  150489:	48 89 f0                                        	mov    %rsi,%rax
  15048c:	48 f7 e3                                        	mul    %rbx
  15048f:	0f 80 c0 fe ff ff                               	jo     150355 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  150495:	48 01 f8                                        	add    %rdi,%rax
  150498:	0f 82 b7 fe ff ff                               	jb     150355 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  15049e:	4d 39 c3                                        	cmp    %r8,%r11
  1504a1:	72 13                                           	jb     1504b6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x186>
  1504a3:	48 39 fb                                        	cmp    %rdi,%rbx
  1504a6:	72 0e                                           	jb     1504b6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x186>
  1504a8:	49 83 c1 e0                                     	add    $0xffffffffffffffe0,%r9
  1504ac:	49 39 42 08                                     	cmp    %rax,0x8(%r10)
  1504b0:	4d 8d 52 20                                     	lea    0x20(%r10),%r10
  1504b4:	73 aa                                           	jae    150460 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x130>
  1504b6:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  1504c0:	48 ff c8                                        	dec    %rax
  1504c3:	49 89 07                                        	mov    %rax,(%r15)
  1504c6:	49 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%r15)
  1504ce:	48 8d 05 8f 32 ec ff                            	lea    -0x13cd71(%rip),%rax        # 13764 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe24>
  1504d5:	49 89 47 18                                     	mov    %rax,0x18(%r15)
  1504d9:	49 c7 47 20 43 00 00 00                         	movq   $0x43,0x20(%r15)
  1504e1:	eb 2b                                           	jmp    15050e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1de>
  1504e3:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  1504ed:	48 ff c8                                        	dec    %rax
  1504f0:	49 89 07                                        	mov    %rax,(%r15)
  1504f3:	49 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%r15)
  1504fb:	48 8d 05 b5 32 ec ff                            	lea    -0x13cd4b(%rip),%rax        # 137b7 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe77>
  150502:	49 89 47 18                                     	mov    %rax,0x18(%r15)
  150506:	49 c7 47 20 33 00 00 00                         	movq   $0x33,0x20(%r15)
  15050e:	66 41 c7 47 28 04 00                            	movw   $0x4,0x28(%r15)
  150515:	41 c6 47 2c 0a                                  	movb   $0xa,0x2c(%r15)
  15051a:	4c 89 f8                                        	mov    %r15,%rax
  15051d:	48 81 c4 18 03 00 00                            	add    $0x318,%rsp
  150524:	5b                                              	pop    %rbx
  150525:	41 5c                                           	pop    %r12
  150527:	41 5d                                           	pop    %r13
  150529:	41 5e                                           	pop    %r14
  15052b:	41 5f                                           	pop    %r15
  15052d:	5d                                              	pop    %rbp
  15052e:	c3                                              	ret
  15052f:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150537:	ba 08 00 00 00                                  	mov    $0x8,%edx
  15053c:	b9 18 00 00 00                                  	mov    $0x18,%ecx
  150541:	48 8b 9c 24 c8 00 00 00                         	mov    0xc8(%rsp),%rbx
  150549:	48 89 de                                        	mov    %rbx,%rsi
  15054c:	e8 af 95 ff ff                                  	call   149b00 <<alloc::raw_vec::RawVecInner>::try_allocate_in>
  150551:	48 8b bc 24 58 01 00 00                         	mov    0x158(%rsp),%rdi
  150559:	80 bc 24 50 01 00 00 00                         	cmpb   $0x0,0x150(%rsp)
  150561:	0f 85 c7 03 00 00                               	jne    15092e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5fe>
  150567:	48 8b 84 24 60 01 00 00                         	mov    0x160(%rsp),%rax
  15056f:	48 89 bc 24 b0 00 00 00                         	mov    %rdi,0xb0(%rsp)
  150577:	48 89 84 24 b8 00 00 00                         	mov    %rax,0xb8(%rsp)
  15057f:	48 c7 84 24 c0 00 00 00 00 00 00 00             	movq   $0x0,0xc0(%rsp)
  15058b:	48 85 db                                        	test   %rbx,%rbx
  15058e:	4c 89 7c 24 08                                  	mov    %r15,0x8(%rsp)
  150593:	0f 84 10 04 00 00                               	je     1509a9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x679>
  150599:	4c 89 f2                                        	mov    %r14,%rdx
  15059c:	41 8d 4c 24 ff                                  	lea    -0x1(%r12),%ecx
  1505a1:	b8 ff ff ff ff                                  	mov    $0xffffffff,%eax
  1505a6:	d3 e0                                           	shl    %cl,%eax
  1505a8:	89 44 24 58                                     	mov    %eax,0x58(%rsp)
  1505ac:	4c 89 a4 24 d8 00 00 00                         	mov    %r12,0xd8(%rsp)
  1505b4:	4c 89 ac 24 d0 00 00 00                         	mov    %r13,0xd0(%rsp)
  1505bc:	eb 53                                           	jmp    150611 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x2e1>
  1505be:	66 90                                           	xchg   %ax,%ax
  1505c0:	48 8b 54 24 30                                  	mov    0x30(%rsp),%rdx
  1505c5:	48 83 c2 20                                     	add    $0x20,%rdx
  1505c9:	48 8b 84 24 b8 00 00 00                         	mov    0xb8(%rsp),%rax
  1505d1:	48 8d 0c 5b                                     	lea    (%rbx,%rbx,2),%rcx
  1505d5:	4c 89 34 c8                                     	mov    %r14,(%rax,%rcx,8)
  1505d9:	48 89 6c c8 08                                  	mov    %rbp,0x8(%rax,%rcx,8)
  1505de:	4c 89 6c c8 10                                  	mov    %r13,0x10(%rax,%rcx,8)
  1505e3:	48 ff c3                                        	inc    %rbx
  1505e6:	48 89 9c 24 c0 00 00 00                         	mov    %rbx,0xc0(%rsp)
  1505ee:	48 3b 94 24 e0 00 00 00                         	cmp    0xe0(%rsp),%rdx
  1505f6:	4c 8b 7c 24 08                                  	mov    0x8(%rsp),%r15
  1505fb:	4c 8b a4 24 d8 00 00 00                         	mov    0xd8(%rsp),%r12
  150603:	4c 8b ac 24 d0 00 00 00                         	mov    0xd0(%rsp),%r13
  15060b:	0f 84 41 02 00 00                               	je     150852 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x522>
  150611:	48 c7 44 24 60 00 00 00 00                      	movq   $0x0,0x60(%rsp)
  15061a:	48 c7 44 24 68 04 00 00 00                      	movq   $0x4,0x68(%rsp)
  150623:	48 c7 44 24 70 00 00 00 00                      	movq   $0x0,0x70(%rsp)
  15062c:	b8 04 00 00 00                                  	mov    $0x4,%eax
  150631:	4d 85 ed                                        	test   %r13,%r13
  150634:	0f 85 89 01 00 00                               	jne    1507c3 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x493>
  15063a:	83 7c 24 28 00                                  	cmpl   $0x0,0x28(%rsp)
  15063f:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
  150644:	0f 84 d2 01 00 00                               	je     15081c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4ec>
  15064a:	83 bc 24 a8 00 00 00 00                         	cmpl   $0x0,0xa8(%rsp)
  150652:	0f 84 c4 01 00 00                               	je     15081c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4ec>
  150658:	48 8b 4a 10                                     	mov    0x10(%rdx),%rcx
  15065c:	48 89 8c 24 e8 00 00 00                         	mov    %rcx,0xe8(%rsp)
  150664:	4c 8b 72 18                                     	mov    0x18(%rdx),%r14
  150668:	41 80 fc 08                                     	cmp    $0x8,%r12b
  15066c:	0f 85 96 00 00 00                               	jne    150708 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x3d8>
  150672:	48 8b 6a 08                                     	mov    0x8(%rdx),%rbp
  150676:	31 db                                           	xor    %ebx,%ebx
  150678:	45 31 ed                                        	xor    %r13d,%r13d
  15067b:	31 c9                                           	xor    %ecx,%ecx
  15067d:	eb 1e                                           	jmp    15069d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x36d>
  15067f:	90                                              	nop
  150680:	48 8b 5c 24 18                                  	mov    0x18(%rsp),%rbx
  150685:	48 03 9c 24 e8 00 00 00                         	add    0xe8(%rsp),%rbx
  15068d:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
  150692:	48 3b 4c 24 28                                  	cmp    0x28(%rsp),%rcx
  150697:	0f 84 82 01 00 00                               	je     15081f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4ef>
  15069d:	48 ff c1                                        	inc    %rcx
  1506a0:	48 89 4c 24 10                                  	mov    %rcx,0x10(%rsp)
  1506a5:	4c 8b a4 24 a8 00 00 00                         	mov    0xa8(%rsp),%r12
  1506ad:	48 89 5c 24 18                                  	mov    %rbx,0x18(%rsp)
  1506b2:	eb 25                                           	jmp    1506d9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x3a9>
  1506b4:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1506c0:	44 03 7c 24 58                                  	add    0x58(%rsp),%r15d
  1506c5:	46 89 3c a8                                     	mov    %r15d,(%rax,%r13,4)
  1506c9:	49 ff c5                                        	inc    %r13
  1506cc:	4c 89 6c 24 70                                  	mov    %r13,0x70(%rsp)
  1506d1:	4c 01 f3                                        	add    %r14,%rbx
  1506d4:	49 ff cc                                        	dec    %r12
  1506d7:	74 a7                                           	je     150680 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x350>
  1506d9:	48 39 eb                                        	cmp    %rbp,%rbx
  1506dc:	0f 83 1f 02 00 00                               	jae    150901 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5d1>
  1506e2:	48 8b 4c 24 30                                  	mov    0x30(%rsp),%rcx
  1506e7:	48 8b 09                                        	mov    (%rcx),%rcx
  1506ea:	44 0f b6 3c 19                                  	movzbl (%rcx,%rbx,1),%r15d
  1506ef:	4c 3b 6c 24 60                                  	cmp    0x60(%rsp),%r13
  1506f4:	75 ca                                           	jne    1506c0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x390>
  1506f6:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  1506fb:	ff 15 2f db 11 00                               	call   *0x11db2f(%rip)        # 26e230 <_DYNAMIC+0x6b0>
  150701:	48 8b 44 24 68                                  	mov    0x68(%rsp),%rax
  150706:	eb b8                                           	jmp    1506c0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x390>
  150708:	48 8b 0a                                        	mov    (%rdx),%rcx
  15070b:	48 89 4c 24 18                                  	mov    %rcx,0x18(%rsp)
  150710:	48 8b 6a 08                                     	mov    0x8(%rdx),%rbp
  150714:	41 bc 01 00 00 00                               	mov    $0x1,%r12d
  15071a:	45 31 ed                                        	xor    %r13d,%r13d
  15071d:	31 c9                                           	xor    %ecx,%ecx
  15071f:	eb 2c                                           	jmp    15074d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x41d>
  150721:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  150730:	4c 8b 64 24 10                                  	mov    0x10(%rsp),%r12
  150735:	4c 03 a4 24 e8 00 00 00                         	add    0xe8(%rsp),%r12
  15073d:	48 8b 4c 24 50                                  	mov    0x50(%rsp),%rcx
  150742:	48 3b 4c 24 28                                  	cmp    0x28(%rsp),%rcx
  150747:	0f 84 d2 00 00 00                               	je     15081f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4ef>
  15074d:	48 ff c1                                        	inc    %rcx
  150750:	48 89 4c 24 50                                  	mov    %rcx,0x50(%rsp)
  150755:	4c 8b bc 24 a8 00 00 00                         	mov    0xa8(%rsp),%r15
  15075d:	4c 89 64 24 10                                  	mov    %r12,0x10(%rsp)
  150762:	eb 24                                           	jmp    150788 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x458>
  150764:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  150770:	03 5c 24 58                                     	add    0x58(%rsp),%ebx
  150774:	42 89 1c a8                                     	mov    %ebx,(%rax,%r13,4)
  150778:	49 ff c5                                        	inc    %r13
  15077b:	4c 89 6c 24 70                                  	mov    %r13,0x70(%rsp)
  150780:	4d 01 f4                                        	add    %r14,%r12
  150783:	49 ff cf                                        	dec    %r15
  150786:	74 a8                                           	je     150730 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x400>
  150788:	49 8d 5c 24 ff                                  	lea    -0x1(%r12),%rbx
  15078d:	48 39 eb                                        	cmp    %rbp,%rbx
  150790:	0f 83 74 01 00 00                               	jae    15090a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5da>
  150796:	49 39 ec                                        	cmp    %rbp,%r12
  150799:	0f 83 74 01 00 00                               	jae    150913 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5e3>
  15079f:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1507a4:	42 0f b7 5c 21 ff                               	movzwl -0x1(%rcx,%r12,1),%ebx
  1507aa:	4c 3b 6c 24 60                                  	cmp    0x60(%rsp),%r13
  1507af:	75 bf                                           	jne    150770 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x440>
  1507b1:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  1507b6:	ff 15 74 da 11 00                               	call   *0x11da74(%rip)        # 26e230 <_DYNAMIC+0x6b0>
  1507bc:	48 8b 44 24 68                                  	mov    0x68(%rsp),%rax
  1507c1:	eb ad                                           	jmp    150770 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x440>
  1507c3:	48 89 d3                                        	mov    %rdx,%rbx
  1507c6:	ba 04 00 00 00                                  	mov    $0x4,%edx
  1507cb:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
  1507d1:	41 b9 04 00 00 00                               	mov    $0x4,%r9d
  1507d7:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  1507df:	31 f6                                           	xor    %esi,%esi
  1507e1:	4c 89 e9                                        	mov    %r13,%rcx
  1507e4:	e8 b7 91 ff ff                                  	call   1499a0 <<alloc::raw_vec::RawVecInner>::finish_grow>
  1507e9:	80 bc 24 50 01 00 00 00                         	cmpb   $0x0,0x150(%rsp)
  1507f1:	0f 85 9f 00 00 00                               	jne    150896 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x566>
  1507f7:	48 8b 84 24 58 01 00 00                         	mov    0x158(%rsp),%rax
  1507ff:	48 89 44 24 68                                  	mov    %rax,0x68(%rsp)
  150804:	4c 89 6c 24 60                                  	mov    %r13,0x60(%rsp)
  150809:	48 89 da                                        	mov    %rbx,%rdx
  15080c:	83 7c 24 28 00                                  	cmpl   $0x0,0x28(%rsp)
  150811:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
  150816:	0f 85 2e fe ff ff                               	jne    15064a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x31a>
  15081c:	45 31 ed                                        	xor    %r13d,%r13d
  15081f:	4c 8b 74 24 60                                  	mov    0x60(%rsp),%r14
  150824:	48 8b 6c 24 68                                  	mov    0x68(%rsp),%rbp
  150829:	48 8b 9c 24 c0 00 00 00                         	mov    0xc0(%rsp),%rbx
  150831:	48 3b 9c 24 b0 00 00 00                         	cmp    0xb0(%rsp),%rbx
  150839:	0f 85 81 fd ff ff                               	jne    1505c0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x290>
  15083f:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
  150847:	ff 15 eb d9 11 00                               	call   *0x11d9eb(%rip)        # 26e238 <_DYNAMIC+0x6b8>
  15084d:	e9 6e fd ff ff                                  	jmp    1505c0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x290>
  150852:	48 83 fb 03                                     	cmp    $0x3,%rbx
  150856:	0f 85 4d 01 00 00                               	jne    1509a9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x679>
  15085c:	48 8b b4 24 b8 00 00 00                         	mov    0xb8(%rsp),%rsi
  150864:	48 8b 46 10                                     	mov    0x10(%rsi),%rax
  150868:	48 3b 46 28                                     	cmp    0x28(%rsi),%rax
  15086c:	75 28                                           	jne    150896 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x566>
  15086e:	48 3b 46 40                                     	cmp    0x40(%rsi),%rax
  150872:	75 22                                           	jne    150896 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x566>
  150874:	48 85 c0                                        	test   %rax,%rax
  150877:	0f 84 2c 01 00 00                               	je     1509a9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x679>
  15087d:	48 8b 4e 08                                     	mov    0x8(%rsi),%rcx
  150881:	48 8b 56 20                                     	mov    0x20(%rsi),%rdx
  150885:	48 8b 76 38                                     	mov    0x38(%rsi),%rsi
  150889:	48 83 f8 04                                     	cmp    $0x4,%rax
  15088d:	73 19                                           	jae    1508a8 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x578>
  15088f:	31 ff                                           	xor    %edi,%edi
  150891:	e9 e2 00 00 00                                  	jmp    150978 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x648>
  150896:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  1508a0:	49 89 07                                        	mov    %rax,(%r15)
  1508a3:	e9 3c 02 00 00                                  	jmp    150ae4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x7b4>
  1508a8:	48 89 c7                                        	mov    %rax,%rdi
  1508ab:	48 83 e7 fc                                     	and    $0xfffffffffffffffc,%rdi
  1508af:	45 31 c0                                        	xor    %r8d,%r8d
  1508b2:	f3 42 0f 6f 04 81                               	movdqu (%rcx,%r8,4),%xmm0
  1508b8:	f3 42 0f 6f 0c 82                               	movdqu (%rdx,%r8,4),%xmm1
  1508be:	f3 42 0f 6f 14 86                               	movdqu (%rsi,%r8,4),%xmm2
  1508c4:	66 0f 6f d8                                     	movdqa %xmm0,%xmm3
  1508c8:	66 0f fe da                                     	paddd  %xmm2,%xmm3
  1508cc:	66 0f fa d1                                     	psubd  %xmm1,%xmm2
  1508d0:	66 0f fa c1                                     	psubd  %xmm1,%xmm0
  1508d4:	66 0f fe c9                                     	paddd  %xmm1,%xmm1
  1508d8:	66 0f fe d9                                     	paddd  %xmm1,%xmm3
  1508dc:	66 0f 72 e3 02                                  	psrad  $0x2,%xmm3
  1508e1:	f3 42 0f 7f 1c 81                               	movdqu %xmm3,(%rcx,%r8,4)
  1508e7:	f3 42 0f 7f 14 82                               	movdqu %xmm2,(%rdx,%r8,4)
  1508ed:	f3 42 0f 7f 04 86                               	movdqu %xmm0,(%rsi,%r8,4)
  1508f3:	49 83 c0 04                                     	add    $0x4,%r8
  1508f7:	4c 39 c7                                        	cmp    %r8,%rdi
  1508fa:	75 b6                                           	jne    1508b2 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x582>
  1508fc:	e9 a3 00 00 00                                  	jmp    1509a4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x674>
  150901:	48 8d 15 80 69 11 00                            	lea    0x116980(%rip),%rdx        # 267288 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2e0>
  150908:	eb 13                                           	jmp    15091d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5ed>
  15090a:	48 8d 15 8f 69 11 00                            	lea    0x11698f(%rip),%rdx        # 2672a0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2f8>
  150911:	eb 0a                                           	jmp    15091d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5ed>
  150913:	4c 89 e3                                        	mov    %r12,%rbx
  150916:	48 8d 15 9b 69 11 00                            	lea    0x11699b(%rip),%rdx        # 2672b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x310>
  15091d:	48 89 df                                        	mov    %rbx,%rdi
  150920:	48 89 ee                                        	mov    %rbp,%rsi
  150923:	ff 15 7f d4 11 00                               	call   *0x11d47f(%rip)        # 26dda8 <_DYNAMIC+0x228>
  150929:	e9 e0 0c 00 00                                  	jmp    15160e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12de>
  15092e:	48 8b b4 24 60 01 00 00                         	mov    0x160(%rsp),%rsi
  150936:	ff 15 5c d4 11 00                               	call   *0x11d45c(%rip)        # 26dd98 <_DYNAMIC+0x218>
  15093c:	49 89 c7                                        	mov    %rax,%r15
  15093f:	4d 85 f6                                        	test   %r14,%r14
  150942:	74 1f                                           	je     150963 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
  150944:	48 89 ef                                        	mov    %rbp,%rdi
  150947:	eb 14                                           	jmp    15095d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x62d>
  150949:	eb 02                                           	jmp    15094d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x61d>
  15094b:	eb 00                                           	jmp    15094d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x61d>
  15094d:	49 89 c7                                        	mov    %rax,%r15
  150950:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
  150956:	74 0b                                           	je     150963 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
  150958:	48 8b 7c 24 68                                  	mov    0x68(%rsp),%rdi
  15095d:	ff 15 6d d4 11 00                               	call   *0x11d46d(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  150963:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
  15096b:	e8 f0 f9 f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  150970:	4c 89 ff                                        	mov    %r15,%rdi
  150973:	e8 38 4b 11 00                                  	call   2654b0 <_Unwind_Resume@plt>
  150978:	44 8b 04 b9                                     	mov    (%rcx,%rdi,4),%r8d
  15097c:	44 8b 0c ba                                     	mov    (%rdx,%rdi,4),%r9d
  150980:	47 8d 14 48                                     	lea    (%r8,%r9,2),%r10d
  150984:	44 8b 1c be                                     	mov    (%rsi,%rdi,4),%r11d
  150988:	45 01 da                                        	add    %r11d,%r10d
  15098b:	41 c1 fa 02                                     	sar    $0x2,%r10d
  15098f:	44 89 14 b9                                     	mov    %r10d,(%rcx,%rdi,4)
  150993:	45 29 cb                                        	sub    %r9d,%r11d
  150996:	44 89 1c ba                                     	mov    %r11d,(%rdx,%rdi,4)
  15099a:	45 29 c8                                        	sub    %r9d,%r8d
  15099d:	44 89 04 be                                     	mov    %r8d,(%rsi,%rdi,4)
  1509a1:	48 ff c7                                        	inc    %rdi
  1509a4:	48 39 f8                                        	cmp    %rdi,%rax
  1509a7:	75 cf                                           	jne    150978 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x648>
  1509a9:	48 c7 84 24 18 01 00 00 00 00 00 00             	movq   $0x0,0x118(%rsp)
  1509b5:	48 c7 84 24 20 01 00 00 04 00 00 00             	movq   $0x4,0x120(%rsp)
  1509c1:	48 c7 84 24 28 01 00 00 00 00 00 00             	movq   $0x0,0x128(%rsp)
  1509cd:	48 8d 84 24 b0 00 00 00                         	lea    0xb0(%rsp),%rax
  1509d5:	48 89 84 24 c0 02 00 00                         	mov    %rax,0x2c0(%rsp)
  1509dd:	4c 8d 74 24 04                                  	lea    0x4(%rsp),%r14
  1509e2:	4c 89 b4 24 c8 02 00 00                         	mov    %r14,0x2c8(%rsp)
  1509ea:	48 8d 44 24 24                                  	lea    0x24(%rsp),%rax
  1509ef:	48 89 84 24 d0 02 00 00                         	mov    %rax,0x2d0(%rsp)
  1509f7:	48 8d 84 24 18 01 00 00                         	lea    0x118(%rsp),%rax
  1509ff:	48 89 84 24 d8 02 00 00                         	mov    %rax,0x2d8(%rsp)
  150a07:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150a0f:	48 8d b4 24 c0 02 00 00                         	lea    0x2c0(%rsp),%rsi
  150a17:	e8 54 2d f7 ff                                  	call   c3770 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}>
  150a1c:	48 83 bc 24 50 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x150(%rsp)
  150a25:	74 4b                                           	je     150a72 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x742>
  150a27:	f3 0f 6f 84 24 50 01 00 00                      	movdqu 0x150(%rsp),%xmm0
  150a30:	f3 0f 6f 8c 24 60 01 00 00                      	movdqu 0x160(%rsp),%xmm1
  150a39:	f3 0f 6f 94 24 70 01 00 00                      	movdqu 0x170(%rsp),%xmm2
  150a42:	f3 41 0f 7f 57 20                               	movdqu %xmm2,0x20(%r15)
  150a48:	f3 41 0f 7f 4f 10                               	movdqu %xmm1,0x10(%r15)
  150a4e:	f3 41 0f 7f 07                                  	movdqu %xmm0,(%r15)
  150a53:	48 83 bc 24 18 01 00 00 00                      	cmpq   $0x0,0x118(%rsp)
  150a5c:	0f 84 82 00 00 00                               	je     150ae4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x7b4>
  150a62:	48 8b bc 24 20 01 00 00                         	mov    0x120(%rsp),%rdi
  150a6a:	ff 15 60 d3 11 00                               	call   *0x11d360(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  150a70:	eb 72                                           	jmp    150ae4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x7b4>
  150a72:	48 83 bc 24 18 01 00 00 00                      	cmpq   $0x0,0x118(%rsp)
  150a7b:	74 0e                                           	je     150a8b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x75b>
  150a7d:	48 8b bc 24 20 01 00 00                         	mov    0x120(%rsp),%rdi
  150a85:	ff 15 45 d3 11 00                               	call   *0x11d345(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  150a8b:	8b 74 24 04                                     	mov    0x4(%rsp),%esi
  150a8f:	8b 54 24 24                                     	mov    0x24(%rsp),%edx
  150a93:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150a9b:	e8 20 1a f8 ff                                  	call   d24c0 <emuella_j2k_codestream::decomp_subband_specs>
  150aa0:	48 8b 84 24 50 01 00 00                         	mov    0x150(%rsp),%rax
  150aa8:	48 8b ac 24 58 01 00 00                         	mov    0x158(%rsp),%rbp
  150ab0:	4c 8b ac 24 60 01 00 00                         	mov    0x160(%rsp),%r13
  150ab8:	4c 8b a4 24 68 01 00 00                         	mov    0x168(%rsp),%r12
  150ac0:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  150ac4:	74 30                                           	je     150af6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x7c6>
  150ac6:	f3 0f 6f 84 24 70 01 00 00                      	movdqu 0x170(%rsp),%xmm0
  150acf:	f3 41 0f 7f 47 20                               	movdqu %xmm0,0x20(%r15)
  150ad5:	49 89 07                                        	mov    %rax,(%r15)
  150ad8:	49 89 6f 08                                     	mov    %rbp,0x8(%r15)
  150adc:	4d 89 6f 10                                     	mov    %r13,0x10(%r15)
  150ae0:	4d 89 67 18                                     	mov    %r12,0x18(%r15)
  150ae4:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
  150aec:	e8 6f f8 f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  150af1:	e9 24 fa ff ff                                  	jmp    15051a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1ea>
  150af6:	48 8b b4 24 b8 00 00 00                         	mov    0xb8(%rsp),%rsi
  150afe:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
  150b06:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  150b0a:	48 8d 14 c6                                     	lea    (%rsi,%rax,8),%rdx
  150b0e:	48 8d 9c 24 f8 01 00 00                         	lea    0x1f8(%rsp),%rbx
  150b16:	48 89 df                                        	mov    %rbx,%rdi
  150b19:	e8 62 fe 01 00                                  	call   170980 <<alloc::vec::Vec<&[i32]> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<&[i32], core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::vec::Vec<i32>>, <alloc::vec::Vec<i32>>::as_slice>>>::from_iter>
  150b1e:	4b 8d 04 a4                                     	lea    (%r12,%r12,4),%rax
  150b22:	48 8d 04 c5 00 00 00 00                         	lea    0x0(,%rax,8),%rax
  150b2a:	4c 01 e8                                        	add    %r13,%rax
  150b2d:	4c 89 6c 24 60                                  	mov    %r13,0x60(%rsp)
  150b32:	48 89 44 24 68                                  	mov    %rax,0x68(%rsp)
  150b37:	4c 89 74 24 70                                  	mov    %r14,0x70(%rsp)
  150b3c:	48 89 5c 24 78                                  	mov    %rbx,0x78(%rsp)
  150b41:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150b49:	48 8d 74 24 60                                  	lea    0x60(%rsp),%rsi
  150b4e:	e8 3d a9 f4 ff                                  	call   9b490 <core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::slice::iter::Iter<emuella_j2k_codestream::DecompSubbandSpec>, emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl<false, false>::{closure#6}>, u8, core::result::Result<core::convert::Infallible, emuella_j2k_codestream::CodestreamError>, <core::result::Result<alloc::vec::Vec<u8>, emuella_j2k_codestream::CodestreamError> as core::iter::traits::collect::FromIterator<core::result::Result<u8, emuella_j2k_codestream::CodestreamError>>>::from_iter<core::iter::adapters::map::Map<core::slice::iter::Iter<emuella_j2k_codestream::DecompSubbandSpec>, emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl<false, false>::{closure#6}>>::{closure#0}, alloc::vec::Vec<u8>>>
  150b53:	48 8b 84 24 50 01 00 00                         	mov    0x150(%rsp),%rax
  150b5b:	48 8b 94 24 58 01 00 00                         	mov    0x158(%rsp),%rdx
  150b63:	48 8b 9c 24 60 01 00 00                         	mov    0x160(%rsp),%rbx
  150b6b:	4c 8b b4 24 68 01 00 00                         	mov    0x168(%rsp),%r14
  150b73:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  150b77:	48 8b 8c 24 58 03 00 00                         	mov    0x358(%rsp),%rcx
  150b7f:	74 23                                           	je     150ba4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x874>
  150b81:	f3 0f 6f 84 24 70 01 00 00                      	movdqu 0x170(%rsp),%xmm0
  150b8a:	f3 41 0f 7f 47 20                               	movdqu %xmm0,0x20(%r15)
  150b90:	49 89 07                                        	mov    %rax,(%r15)
  150b93:	49 89 57 08                                     	mov    %rdx,0x8(%r15)
  150b97:	49 89 5f 10                                     	mov    %rbx,0x10(%r15)
  150b9b:	4d 89 77 18                                     	mov    %r14,0x18(%r15)
  150b9f:	e9 e1 00 00 00                                  	jmp    150c85 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x955>
  150ba4:	48 89 54 24 58                                  	mov    %rdx,0x58(%rsp)
  150ba9:	48 c7 44 24 38 00 00 00 00                      	movq   $0x0,0x38(%rsp)
  150bb2:	48 c7 44 24 40 01 00 00 00                      	movq   $0x1,0x40(%rsp)
  150bbb:	48 c7 44 24 48 00 00 00 00                      	movq   $0x0,0x48(%rsp)
  150bc4:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150bcc:	48 8d 74 24 38                                  	lea    0x38(%rsp),%rsi
  150bd1:	ba 80 00 00 00                                  	mov    $0x80,%edx
  150bd6:	e8 15 f6 ff ff                                  	call   1501f0 <emuella_j2k_codestream::scalable_lossless::reserve_output>
  150bdb:	48 83 bc 24 50 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x150(%rsp)
  150be4:	75 4f                                           	jne    150c35 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x905>
  150be6:	31 c0                                           	xor    %eax,%eax
  150be8:	4c 8b 94 24 c8 00 00 00                         	mov    0xc8(%rsp),%r10
  150bf0:	49 83 fa 03                                     	cmp    $0x3,%r10
  150bf4:	0f 94 c0                                        	sete   %al
  150bf7:	8b 54 24 04                                     	mov    0x4(%rsp),%edx
  150bfb:	8b 4c 24 24                                     	mov    0x24(%rsp),%ecx
  150bff:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  150c07:	48 8d 74 24 38                                  	lea    0x38(%rsp),%rsi
  150c0c:	41 89 d0                                        	mov    %edx,%r8d
  150c0f:	41 89 c9                                        	mov    %ecx,%r9d
  150c12:	6a 00                                           	push   $0x0
  150c14:	41 56                                           	push   %r14
  150c16:	53                                              	push   %rbx
  150c17:	50                                              	push   %rax
  150c18:	41 52                                           	push   %r10
  150c1a:	ff b4 24 68 02 00 00                            	push   0x268(%rsp)
  150c21:	e8 1a 56 f8 ff                                  	call   d6240 <emuella_j2k_codestream::write_native_main_header>
  150c26:	48 83 c4 30                                     	add    $0x30,%rsp
  150c2a:	48 83 bc 24 50 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x150(%rsp)
  150c33:	74 7a                                           	je     150caf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x97f>
  150c35:	f3 0f 6f 84 24 50 01 00 00                      	movdqu 0x150(%rsp),%xmm0
  150c3e:	f3 0f 6f 8c 24 60 01 00 00                      	movdqu 0x160(%rsp),%xmm1
  150c47:	f3 0f 6f 94 24 70 01 00 00                      	movdqu 0x170(%rsp),%xmm2
  150c50:	f3 41 0f 7f 57 20                               	movdqu %xmm2,0x20(%r15)
  150c56:	f3 41 0f 7f 4f 10                               	movdqu %xmm1,0x10(%r15)
  150c5c:	f3 41 0f 7f 07                                  	movdqu %xmm0,(%r15)
  150c61:	48 83 7c 24 38 00                               	cmpq   $0x0,0x38(%rsp)
  150c67:	74 0b                                           	je     150c74 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x944>
  150c69:	48 8b 7c 24 40                                  	mov    0x40(%rsp),%rdi
  150c6e:	ff 15 5c d1 11 00                               	call   *0x11d15c(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  150c74:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
  150c7a:	74 09                                           	je     150c85 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x955>
  150c7c:	48 89 df                                        	mov    %rbx,%rdi
  150c7f:	ff 15 4b d1 11 00                               	call   *0x11d14b(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  150c85:	48 83 bc 24 f8 01 00 00 00                      	cmpq   $0x0,0x1f8(%rsp)
  150c8e:	74 0e                                           	je     150c9e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x96e>
  150c90:	48 8b bc 24 00 02 00 00                         	mov    0x200(%rsp),%rdi
  150c98:	ff 15 32 d1 11 00                               	call   *0x11d132(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  150c9e:	48 85 ed                                        	test   %rbp,%rbp
  150ca1:	0f 84 3d fe ff ff                               	je     150ae4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x7b4>
  150ca7:	4c 89 ef                                        	mov    %r13,%rdi
  150caa:	e9 bb fd ff ff                                  	jmp    150a6a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x73a>
  150caf:	48 8b 44 24 48                                  	mov    0x48(%rsp),%rax
  150cb4:	48 89 84 24 e0 00 00 00                         	mov    %rax,0xe0(%rsp)
  150cbc:	48 8d 35 e4 2a ec ff                            	lea    -0x13d51c(%rip),%rsi        # 137a7 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe67>
  150cc3:	48 8d 7c 24 38                                  	lea    0x38(%rsp),%rdi
  150cc8:	ba 0e 00 00 00                                  	mov    $0xe,%edx
  150ccd:	e8 6e bd ff ff                                  	call   14ca40 <<alloc::vec::Vec<u8>>::append_elements>
  150cd2:	48 89 5c 24 18                                  	mov    %rbx,0x18(%rsp)
  150cd7:	48 89 6c 24 10                                  	mov    %rbp,0x10(%rsp)
  150cdc:	48 c7 84 24 50 01 00 00 00 00 00 00             	movq   $0x0,0x150(%rsp)
  150ce8:	48 c7 84 24 58 01 00 00 02 00 00 00             	movq   $0x2,0x158(%rsp)
  150cf4:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
  150cf8:	f3 0f 7f 84 24 60 01 00 00                      	movdqu %xmm0,0x160(%rsp)
  150d01:	48 c7 84 24 70 01 00 00 01 00 00 00             	movq   $0x1,0x170(%rsp)
  150d0d:	f3 0f 7f 84 24 78 01 00 00                      	movdqu %xmm0,0x178(%rsp)
  150d16:	48 c7 84 24 88 01 00 00 04 00 00 00             	movq   $0x4,0x188(%rsp)
  150d22:	f3 0f 7f 84 24 90 01 00 00                      	movdqu %xmm0,0x190(%rsp)
  150d2b:	48 c7 84 24 a0 01 00 00 08 00 00 00             	movq   $0x8,0x1a0(%rsp)
  150d37:	f3 0f 7f 84 24 a8 01 00 00                      	movdqu %xmm0,0x1a8(%rsp)
  150d40:	48 c7 84 24 b8 01 00 00 01 00 00 00             	movq   $0x1,0x1b8(%rsp)
  150d4c:	f3 0f 7f 84 24 c0 01 00 00                      	movdqu %xmm0,0x1c0(%rsp)
  150d55:	48 c7 84 24 d0 01 00 00 01 00 00 00             	movq   $0x1,0x1d0(%rsp)
  150d61:	f3 0f 7f 84 24 d8 01 00 00                      	movdqu %xmm0,0x1d8(%rsp)
  150d6a:	48 c7 84 24 e8 01 00 00 04 00 00 00             	movq   $0x4,0x1e8(%rsp)
  150d76:	48 c7 84 24 f0 01 00 00 00 00 00 00             	movq   $0x0,0x1f0(%rsp)
  150d82:	31 f6                                           	xor    %esi,%esi
  150d84:	48 8b 84 24 48 01 00 00                         	mov    0x148(%rsp),%rax
  150d8c:	48 83 f8 02                                     	cmp    $0x2,%rax
  150d90:	48 0f 43 f0                                     	cmovae %rax,%rsi
  150d94:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  150d99:	e8 12 39 ff ff                                  	call   1446b0 <<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>::new>
  150d9e:	8b 44 24 60                                     	mov    0x60(%rsp),%eax
  150da2:	0f 10 44 24 68                                  	movups 0x68(%rsp),%xmm0
  150da7:	0f 29 84 24 10 02 00 00                         	movaps %xmm0,0x210(%rsp)
  150daf:	0f 10 44 24 78                                  	movups 0x78(%rsp),%xmm0
  150db4:	0f 29 84 24 20 02 00 00                         	movaps %xmm0,0x220(%rsp)
  150dbc:	0f 10 84 24 88 00 00 00                         	movups 0x88(%rsp),%xmm0
  150dc4:	0f 29 84 24 30 02 00 00                         	movaps %xmm0,0x230(%rsp)
  150dcc:	83 f8 01                                        	cmp    $0x1,%eax
  150dcf:	75 33                                           	jne    150e04 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xad4>
  150dd1:	66 0f 6f 84 24 10 02 00 00                      	movdqa 0x210(%rsp),%xmm0
  150dda:	66 0f 6f 8c 24 20 02 00 00                      	movdqa 0x220(%rsp),%xmm1
  150de3:	66 0f 6f 94 24 30 02 00 00                      	movdqa 0x230(%rsp),%xmm2
  150dec:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  150df1:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  150df6:	f3 0f 7f 48 10                                  	movdqu %xmm1,0x10(%rax)
  150dfb:	f3 0f 7f 00                                     	movdqu %xmm0,(%rax)
  150dff:	e9 00 07 00 00                                  	jmp    151504 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11d4>
  150e04:	66 0f 6f 84 24 10 02 00 00                      	movdqa 0x210(%rsp),%xmm0
  150e0d:	66 0f 6f 8c 24 20 02 00 00                      	movdqa 0x220(%rsp),%xmm1
  150e16:	66 0f 6f 94 24 30 02 00 00                      	movdqa 0x230(%rsp),%xmm2
  150e1f:	66 0f 7f 94 24 b0 02 00 00                      	movdqa %xmm2,0x2b0(%rsp)
  150e28:	66 0f 7f 8c 24 a0 02 00 00                      	movdqa %xmm1,0x2a0(%rsp)
  150e31:	66 0f 7f 84 24 90 02 00 00                      	movdqa %xmm0,0x290(%rsp)
  150e3a:	4d 39 e6                                        	cmp    %r12,%r14
  150e3d:	4d 0f 42 e6                                     	cmovb  %r14,%r12
  150e41:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
  150e49:	48 85 c0                                        	test   %rax,%rax
  150e4c:	0f 84 48 05 00 00                               	je     15139a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x106a>
  150e52:	48 8b 8c 24 98 02 00 00                         	mov    0x298(%rsp),%rcx
  150e5a:	48 89 8c 24 a8 00 00 00                         	mov    %rcx,0xa8(%rsp)
  150e62:	48 8b 8c 24 a0 02 00 00                         	mov    0x2a0(%rsp),%rcx
  150e6a:	48 89 8c 24 e8 00 00 00                         	mov    %rcx,0xe8(%rsp)
  150e72:	48 c7 44 24 30 00 00 00 00                      	movq   $0x0,0x30(%rsp)
  150e7b:	48 8b 54 24 30                                  	mov    0x30(%rsp),%rdx
  150e80:	8d 4a 01                                        	lea    0x1(%rdx),%ecx
  150e83:	80 fa 02                                        	cmp    $0x2,%dl
  150e86:	0f b6 d1                                        	movzbl %cl,%edx
  150e89:	b9 02 00 00 00                                  	mov    $0x2,%ecx
  150e8e:	0f 44 d1                                        	cmove  %ecx,%edx
  150e91:	89 94 24 d0 00 00 00                            	mov    %edx,0xd0(%rsp)
  150e98:	48 85 c0                                        	test   %rax,%rax
  150e9b:	0f 84 cf 04 00 00                               	je     151370 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1040>
  150ea1:	48 8b 8c 24 b8 00 00 00                         	mov    0xb8(%rsp),%rcx
  150ea9:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  150ead:	48 89 4c 24 28                                  	mov    %rcx,0x28(%rsp)
  150eb2:	48 8d 04 c1                                     	lea    (%rcx,%rax,8),%rax
  150eb6:	48 89 84 24 c8 00 00 00                         	mov    %rax,0xc8(%rsp)
  150ebe:	48 8b 44 24 48                                  	mov    0x48(%rsp),%rax
  150ec3:	48 89 44 24 50                                  	mov    %rax,0x50(%rsp)
  150ec8:	bf 90 00 00 00                                  	mov    $0x90,%edi
  150ecd:	ff 15 15 cf 11 00                               	call   *0x11cf15(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
  150ed3:	48 85 c0                                        	test   %rax,%rax
  150ed6:	0f 84 22 07 00 00                               	je     1515fe <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12ce>
  150edc:	48 c7 84 24 30 01 00 00 03 00 00 00             	movq   $0x3,0x130(%rsp)
  150ee8:	48 89 84 24 38 01 00 00                         	mov    %rax,0x138(%rsp)
  150ef0:	48 c7 84 24 40 01 00 00 00 00 00 00             	movq   $0x0,0x140(%rsp)
  150efc:	48 c7 84 24 48 02 00 00 00 00 00 00             	movq   $0x0,0x248(%rsp)
  150f08:	48 c7 84 24 50 02 00 00 08 00 00 00             	movq   $0x8,0x250(%rsp)
  150f14:	48 c7 84 24 58 02 00 00 00 00 00 00             	movq   $0x0,0x258(%rsp)
  150f20:	48 83 bc 24 48 01 00 00 01                      	cmpq   $0x1,0x148(%rsp)
  150f29:	0f 86 4d 01 00 00                               	jbe    15107c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd4c>
  150f2f:	4d 89 ef                                        	mov    %r13,%r15
  150f32:	31 db                                           	xor    %ebx,%ebx
  150f34:	48 8d ac 24 f0 00 00 00                         	lea    0xf0(%rsp),%rbp
  150f3c:	eb 07                                           	jmp    150f45 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xc15>
  150f3e:	49 83 c7 28                                     	add    $0x28,%r15
  150f42:	48 ff c3                                        	inc    %rbx
  150f45:	4c 39 e3                                        	cmp    %r12,%rbx
  150f48:	0f 83 18 02 00 00                               	jae    151166 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe36>
  150f4e:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  150f53:	41 38 47 21                                     	cmp    %al,0x21(%r15)
  150f57:	75 e5                                           	jne    150f3e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xc0e>
  150f59:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  150f5e:	4c 8b 40 08                                     	mov    0x8(%rax),%r8
  150f62:	4c 8b 48 10                                     	mov    0x10(%rax),%r9
  150f66:	49 8b 47 20                                     	mov    0x20(%r15),%rax
  150f6a:	48 89 84 24 10 01 00 00                         	mov    %rax,0x110(%rsp)
  150f72:	f3 41 0f 6f 07                                  	movdqu (%r15),%xmm0
  150f77:	f3 41 0f 6f 4f 10                               	movdqu 0x10(%r15),%xmm1
  150f7d:	66 0f 7f 8c 24 00 01 00 00                      	movdqa %xmm1,0x100(%rsp)
  150f86:	66 0f 7f 84 24 f0 00 00 00                      	movdqa %xmm0,0xf0(%rsp)
  150f8f:	8b 4c 24 04                                     	mov    0x4(%rsp),%ecx
  150f93:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  150f98:	0f b6 04 18                                     	movzbl (%rax,%rbx,1),%eax
  150f9c:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  150fa1:	48 8b b4 24 a8 00 00 00                         	mov    0xa8(%rsp),%rsi
  150fa9:	48 8b 94 24 e8 00 00 00                         	mov    0xe8(%rsp),%rdx
  150fb1:	ff b4 24 58 03 00 00                            	push   0x358(%rsp)
  150fb8:	4c 8d 54 24 40                                  	lea    0x40(%rsp),%r10
  150fbd:	41 52                                           	push   %r10
  150fbf:	50                                              	push   %rax
  150fc0:	55                                              	push   %rbp
  150fc1:	e8 0a 55 f3 ff                                  	call   864d0 <<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>::encode_subband::<false, false>>
  150fc6:	48 83 c4 20                                     	add    $0x20,%rsp
  150fca:	48 8b 44 24 60                                  	mov    0x60(%rsp),%rax
  150fcf:	48 8b ac 24 90 00 00 00                         	mov    0x90(%rsp),%rbp
  150fd7:	48 8d 4c 24 68                                  	lea    0x68(%rsp),%rcx
  150fdc:	0f 10 01                                        	movups (%rcx),%xmm0
  150fdf:	0f 10 49 10                                     	movups 0x10(%rcx),%xmm1
  150fe3:	0f 29 84 24 60 02 00 00                         	movaps %xmm0,0x260(%rsp)
  150feb:	0f 29 8c 24 70 02 00 00                         	movaps %xmm1,0x270(%rsp)
  150ff3:	48 8b 49 20                                     	mov    0x20(%rcx),%rcx
  150ff7:	48 89 8c 24 80 02 00 00                         	mov    %rcx,0x280(%rsp)
  150fff:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  151003:	0f 84 a8 04 00 00                               	je     1514b1 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1181>
  151009:	4c 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%r14
  151011:	48 89 84 24 10 02 00 00                         	mov    %rax,0x210(%rsp)
  151019:	48 8b 84 24 80 02 00 00                         	mov    0x280(%rsp),%rax
  151021:	48 8d 8c 24 18 02 00 00                         	lea    0x218(%rsp),%rcx
  151029:	48 89 41 20                                     	mov    %rax,0x20(%rcx)
  15102d:	66 0f 6f 84 24 60 02 00 00                      	movdqa 0x260(%rsp),%xmm0
  151036:	66 0f 6f 8c 24 70 02 00 00                      	movdqa 0x270(%rsp),%xmm1
  15103f:	f3 0f 7f 49 10                                  	movdqu %xmm1,0x10(%rcx)
  151044:	f3 0f 7f 01                                     	movdqu %xmm0,(%rcx)
  151048:	48 8d bc 24 30 01 00 00                         	lea    0x130(%rsp),%rdi
  151050:	48 8d b4 24 10 02 00 00                         	lea    0x210(%rsp),%rsi
  151058:	e8 43 95 ff ff                                  	call   14a5a0 <<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>::push_mut>
  15105d:	48 85 ed                                        	test   %rbp,%rbp
  151060:	48 8d ac 24 f0 00 00 00                         	lea    0xf0(%rsp),%rbp
  151068:	0f 84 d0 fe ff ff                               	je     150f3e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xc0e>
  15106e:	4c 89 f7                                        	mov    %r14,%rdi
  151071:	ff 15 59 cd 11 00                               	call   *0x11cd59(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  151077:	e9 c2 fe ff ff                                  	jmp    150f3e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xc0e>
  15107c:	4c 89 eb                                        	mov    %r13,%rbx
  15107f:	45 31 f6                                        	xor    %r14d,%r14d
  151082:	4c 8d bc 24 f0 00 00 00                         	lea    0xf0(%rsp),%r15
  15108a:	eb 07                                           	jmp    151093 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd63>
  15108c:	48 83 c3 28                                     	add    $0x28,%rbx
  151090:	49 ff c6                                        	inc    %r14
  151093:	4d 39 e6                                        	cmp    %r12,%r14
  151096:	0f 83 ca 00 00 00                               	jae    151166 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe36>
  15109c:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1510a1:	38 43 21                                        	cmp    %al,0x21(%rbx)
  1510a4:	75 e6                                           	jne    15108c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd5c>
  1510a6:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1510ab:	48 8b 50 08                                     	mov    0x8(%rax),%rdx
  1510af:	48 8b 48 10                                     	mov    0x10(%rax),%rcx
  1510b3:	48 8b 43 20                                     	mov    0x20(%rbx),%rax
  1510b7:	48 89 84 24 10 01 00 00                         	mov    %rax,0x110(%rsp)
  1510bf:	f3 0f 6f 03                                     	movdqu (%rbx),%xmm0
  1510c3:	f3 0f 6f 4b 10                                  	movdqu 0x10(%rbx),%xmm1
  1510c8:	66 0f 7f 8c 24 00 01 00 00                      	movdqa %xmm1,0x100(%rsp)
  1510d1:	66 0f 7f 84 24 f0 00 00 00                      	movdqa %xmm0,0xf0(%rsp)
  1510da:	8b 74 24 04                                     	mov    0x4(%rsp),%esi
  1510de:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  1510e3:	46 0f b6 0c 30                                  	movzbl (%rax,%r14,1),%r9d
  1510e8:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  1510ed:	4d 89 f8                                        	mov    %r15,%r8
  1510f0:	ff b4 24 58 03 00 00                            	push   0x358(%rsp)
  1510f7:	6a 01                                           	push   $0x1
  1510f9:	48 8d 84 24 60 01 00 00                         	lea    0x160(%rsp),%rax
  151101:	50                                              	push   %rax
  151102:	48 8d 44 24 50                                  	lea    0x50(%rsp),%rax
  151107:	50                                              	push   %rax
  151108:	e8 d3 42 f3 ff                                  	call   853e0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>>
  15110d:	48 83 c4 20                                     	add    $0x20,%rsp
  151111:	8b 44 24 60                                     	mov    0x60(%rsp),%eax
  151115:	48 8d 4c 24 68                                  	lea    0x68(%rsp),%rcx
  15111a:	f3 0f 6f 01                                     	movdqu (%rcx),%xmm0
  15111e:	f3 0f 6f 49 10                                  	movdqu 0x10(%rcx),%xmm1
  151123:	f3 0f 6f 51 20                                  	movdqu 0x20(%rcx),%xmm2
  151128:	66 0f 7f 84 24 e0 02 00 00                      	movdqa %xmm0,0x2e0(%rsp)
  151131:	66 0f 7f 8c 24 f0 02 00 00                      	movdqa %xmm1,0x2f0(%rsp)
  15113a:	66 0f 7f 94 24 00 03 00 00                      	movdqa %xmm2,0x300(%rsp)
  151143:	83 f8 01                                        	cmp    $0x1,%eax
  151146:	0f 84 39 03 00 00                               	je     151485 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1155>
  15114c:	48 8d bc 24 30 01 00 00                         	lea    0x130(%rsp),%rdi
  151154:	48 8d b4 24 e0 02 00 00                         	lea    0x2e0(%rsp),%rsi
  15115c:	e8 3f 94 ff ff                                  	call   14a5a0 <<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>::push_mut>
  151161:	e9 26 ff ff ff                                  	jmp    15108c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd5c>
  151166:	48 c7 84 24 f0 00 00 00 00 00 00 00             	movq   $0x0,0xf0(%rsp)
  151172:	48 c7 84 24 f8 00 00 00 01 00 00 00             	movq   $0x1,0xf8(%rsp)
  15117e:	48 8d 84 24 00 01 00 00                         	lea    0x100(%rsp),%rax
  151186:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
  15118a:	f3 0f 7f 00                                     	movdqu %xmm0,(%rax)
  15118e:	4c 8b b4 24 38 01 00 00                         	mov    0x138(%rsp),%r14
  151196:	48 8b 9c 24 40 01 00 00                         	mov    0x140(%rsp),%rbx
  15119e:	48 89 d8                                        	mov    %rbx,%rax
  1511a1:	48 c1 e0 04                                     	shl    $0x4,%rax
  1511a5:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  1511a9:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1511b1:	48 85 db                                        	test   %rbx,%rbx
  1511b4:	74 38                                           	je     1511ee <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xebe>
  1511b6:	48 8b 84 24 d8 00 00 00                         	mov    0xd8(%rsp),%rax
  1511be:	4c 01 f0                                        	add    %r14,%rax
  1511c1:	4c 89 f1                                        	mov    %r14,%rcx
  1511c4:	48 8b 71 08                                     	mov    0x8(%rcx),%rsi
  1511c8:	48 8b 51 10                                     	mov    0x10(%rcx),%rdx
  1511cc:	48 83 c1 30                                     	add    $0x30,%rcx
  1511d0:	48 c1 e2 05                                     	shl    $0x5,%rdx
  1511d4:	48 85 d2                                        	test   %rdx,%rdx
  1511d7:	74 10                                           	je     1511e9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xeb9>
  1511d9:	48 83 c2 e0                                     	add    $0xffffffffffffffe0,%rdx
  1511dd:	80 7e 1b 00                                     	cmpb   $0x0,0x1b(%rsi)
  1511e1:	48 8d 76 20                                     	lea    0x20(%rsi),%rsi
  1511e5:	74 ed                                           	je     1511d4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xea4>
  1511e7:	eb 09                                           	jmp    1511f2 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xec2>
  1511e9:	48 39 c1                                        	cmp    %rax,%rcx
  1511ec:	75 d6                                           	jne    1511c4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe94>
  1511ee:	31 ed                                           	xor    %ebp,%ebp
  1511f0:	eb 03                                           	jmp    1511f5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xec5>
  1511f2:	40 b5 01                                        	mov    $0x1,%bpl
  1511f5:	40 0f b6 d5                                     	movzbl %bpl,%edx
  1511f9:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  1511fe:	48 8d b4 24 f0 00 00 00                         	lea    0xf0(%rsp),%rsi
  151206:	e8 45 a9 ff ff                                  	call   14bb50 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
  15120b:	48 83 7c 24 60 ff                               	cmpq   $0xffffffffffffffff,0x60(%rsp)
  151211:	0f 85 b3 01 00 00                               	jne    1513ca <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x109a>
  151217:	48 85 db                                        	test   %rbx,%rbx
  15121a:	0f 94 c0                                        	sete   %al
  15121d:	40 80 f5 01                                     	xor    $0x1,%bpl
  151221:	40 08 c5                                        	or     %al,%bpl
  151224:	4c 8d bc 24 f0 00 00 00                         	lea    0xf0(%rsp),%r15
  15122c:	48 8b ac 24 d8 00 00 00                         	mov    0xd8(%rsp),%rbp
  151234:	75 39                                           	jne    15126f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xf3f>
  151236:	31 db                                           	xor    %ebx,%ebx
  151238:	4d 8b 44 1e 08                                  	mov    0x8(%r14,%rbx,1),%r8
  15123d:	4d 8b 4c 1e 10                                  	mov    0x10(%r14,%rbx,1),%r9
  151242:	41 0f b7 4c 1e 2a                               	movzwl 0x2a(%r14,%rbx,1),%ecx
  151248:	41 8b 54 1e 28                                  	mov    0x28(%r14,%rbx,1),%edx
  15124d:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  151252:	4c 89 fe                                        	mov    %r15,%rsi
  151255:	e8 86 2d f9 ff                                  	call   e3fe0 <emuella_j2k_codestream::write_component_packet_header>
  15125a:	48 83 7c 24 60 ff                               	cmpq   $0xffffffffffffffff,0x60(%rsp)
  151260:	0f 85 64 01 00 00                               	jne    1513ca <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x109a>
  151266:	48 83 c3 30                                     	add    $0x30,%rbx
  15126a:	48 39 dd                                        	cmp    %rbx,%rbp
  15126d:	75 c9                                           	jne    151238 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xf08>
  15126f:	48 8b 84 24 08 01 00 00                         	mov    0x108(%rsp),%rax
  151277:	a8 07                                           	test   $0x7,%al
  151279:	74 10                                           	je     15128b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xf5b>
  15127b:	48 83 e0 f8                                     	and    $0xfffffffffffffff8,%rax
  15127f:	48 83 c0 08                                     	add    $0x8,%rax
  151283:	48 89 84 24 08 01 00 00                         	mov    %rax,0x108(%rsp)
  15128b:	48 8b 9c 24 f8 00 00 00                         	mov    0xf8(%rsp),%rbx
  151293:	4c 8b bc 24 00 01 00 00                         	mov    0x100(%rsp),%r15
  15129b:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  1512a0:	48 8d 74 24 38                                  	lea    0x38(%rsp),%rsi
  1512a5:	4c 89 fa                                        	mov    %r15,%rdx
  1512a8:	48 8b 8c 24 58 03 00 00                         	mov    0x358(%rsp),%rcx
  1512b0:	e8 3b ef ff ff                                  	call   1501f0 <emuella_j2k_codestream::scalable_lossless::reserve_output>
  1512b5:	48 83 7c 24 60 ff                               	cmpq   $0xffffffffffffffff,0x60(%rsp)
  1512bb:	0f 85 09 01 00 00                               	jne    1513ca <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x109a>
  1512c1:	48 8b 6c 24 48                                  	mov    0x48(%rsp),%rbp
  1512c6:	49 8d 34 2f                                     	lea    (%r15,%rbp,1),%rsi
  1512ca:	48 8d 7c 24 38                                  	lea    0x38(%rsp),%rdi
  1512cf:	e8 1c 3d ff ff                                  	call   144ff0 <<alloc::vec::Vec<u8>>::resize>
  1512d4:	48 8b 7c 24 40                                  	mov    0x40(%rsp),%rdi
  1512d9:	48 8b 74 24 48                                  	mov    0x48(%rsp),%rsi
  1512de:	48 8b 54 24 50                                  	mov    0x50(%rsp),%rdx
  1512e3:	4d 8d 34 17                                     	lea    (%r15,%rdx,1),%r14
  1512e7:	48 89 e9                                        	mov    %rbp,%rcx
  1512ea:	4d 89 f0                                        	mov    %r14,%r8
  1512ed:	e8 ee 50 f3 ff                                  	call   863e0 <<[u8]>::copy_within::<core::ops::range::Range<usize>>>
  1512f2:	48 8b 54 24 48                                  	mov    0x48(%rsp),%rdx
  1512f7:	4c 3b 74 24 50                                  	cmp    0x50(%rsp),%r14
  1512fc:	0f 82 d3 02 00 00                               	jb     1515d5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12a5>
  151302:	49 39 d6                                        	cmp    %rdx,%r14
  151305:	0f 87 ca 02 00 00                               	ja     1515d5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12a5>
  15130b:	48 8b 7c 24 50                                  	mov    0x50(%rsp),%rdi
  151310:	48 03 7c 24 40                                  	add    0x40(%rsp),%rdi
  151315:	48 89 de                                        	mov    %rbx,%rsi
  151318:	4c 89 fa                                        	mov    %r15,%rdx
  15131b:	ff 15 97 ca 11 00                               	call   *0x11ca97(%rip)        # 26ddb8 <memcpy@GLIBC_2.14>
  151321:	48 83 bc 24 f0 00 00 00 00                      	cmpq   $0x0,0xf0(%rsp)
  15132a:	74 0e                                           	je     15133a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x100a>
  15132c:	48 8b bc 24 f8 00 00 00                         	mov    0xf8(%rsp),%rdi
  151334:	ff 15 96 ca 11 00                               	call   *0x11ca96(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  15133a:	48 8b 5c 24 28                                  	mov    0x28(%rsp),%rbx
  15133f:	48 83 c3 18                                     	add    $0x18,%rbx
  151343:	48 8d bc 24 48 02 00 00                         	lea    0x248(%rsp),%rdi
  15134b:	e8 10 f0 f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  151350:	48 8d bc 24 30 01 00 00                         	lea    0x130(%rsp),%rdi
  151358:	e8 d3 f6 f3 ff                                  	call   90a30 <core::ptr::drop_glue::<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>>
  15135d:	48 89 5c 24 28                                  	mov    %rbx,0x28(%rsp)
  151362:	48 3b 9c 24 c8 00 00 00                         	cmp    0xc8(%rsp),%rbx
  15136a:	0f 85 4e fb ff ff                               	jne    150ebe <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xb8e>
  151370:	80 7c 24 30 02                                  	cmpb   $0x2,0x30(%rsp)
  151375:	74 23                                           	je     15139a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x106a>
  151377:	80 bc 24 d0 00 00 00 02                         	cmpb   $0x2,0xd0(%rsp)
  15137f:	77 19                                           	ja     15139a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x106a>
  151381:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
  151389:	8b 8c 24 d0 00 00 00                            	mov    0xd0(%rsp),%ecx
  151390:	48 89 4c 24 30                                  	mov    %rcx,0x30(%rsp)
  151395:	e9 e1 fa ff ff                                  	jmp    150e7b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xb4b>
  15139a:	48 8b 54 24 48                                  	mov    0x48(%rsp),%rdx
  15139f:	48 89 d0                                        	mov    %rdx,%rax
  1513a2:	48 2b 84 24 e0 00 00 00                         	sub    0xe0(%rsp),%rax
  1513aa:	48 89 c1                                        	mov    %rax,%rcx
  1513ad:	48 c1 e9 20                                     	shr    $0x20,%rcx
  1513b1:	74 5d                                           	je     151410 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x10e0>
  1513b3:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  1513bd:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
  1513c2:	48 89 01                                        	mov    %rax,(%rcx)
  1513c5:	e9 2d 01 00 00                                  	jmp    1514f7 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11c7>
  1513ca:	0f 10 44 24 60                                  	movups 0x60(%rsp),%xmm0
  1513cf:	0f 10 4c 24 70                                  	movups 0x70(%rsp),%xmm1
  1513d4:	f3 0f 6f 94 24 80 00 00 00                      	movdqu 0x80(%rsp),%xmm2
  1513dd:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  1513e2:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  1513e7:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
  1513eb:	0f 11 00                                        	movups %xmm0,(%rax)
  1513ee:	48 83 bc 24 f0 00 00 00 00                      	cmpq   $0x0,0xf0(%rsp)
  1513f7:	0f 84 e0 00 00 00                               	je     1514dd <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11ad>
  1513fd:	48 8b bc 24 f8 00 00 00                         	mov    0xf8(%rsp),%rdi
  151405:	ff 15 c5 c9 11 00                               	call   *0x11c9c5(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  15140b:	e9 cd 00 00 00                                  	jmp    1514dd <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11ad>
  151410:	48 8b 8c 24 e0 00 00 00                         	mov    0xe0(%rsp),%rcx
  151418:	48 8d 79 06                                     	lea    0x6(%rcx),%rdi
  15141c:	48 83 c1 0a                                     	add    $0xa,%rcx
  151420:	48 39 d1                                        	cmp    %rdx,%rcx
  151423:	0f 87 c3 01 00 00                               	ja     1515ec <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12bc>
  151429:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
  15142e:	0f c8                                           	bswap  %eax
  151430:	89 04 39                                        	mov    %eax,(%rcx,%rdi,1)
  151433:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
  151438:	48 8d 74 24 38                                  	lea    0x38(%rsp),%rsi
  15143d:	ba 02 00 00 00                                  	mov    $0x2,%edx
  151442:	48 8b 8c 24 58 03 00 00                         	mov    0x358(%rsp),%rcx
  15144a:	e8 a1 ed ff ff                                  	call   1501f0 <emuella_j2k_codestream::scalable_lossless::reserve_output>
  15144f:	48 83 7c 24 60 ff                               	cmpq   $0xffffffffffffffff,0x60(%rsp)
  151455:	0f 84 d6 00 00 00                               	je     151531 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1201>
  15145b:	f3 0f 6f 44 24 60                               	movdqu 0x60(%rsp),%xmm0
  151461:	f3 0f 6f 4c 24 70                               	movdqu 0x70(%rsp),%xmm1
  151467:	f3 0f 6f 94 24 80 00 00 00                      	movdqu 0x80(%rsp),%xmm2
  151470:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  151475:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  15147a:	f3 0f 7f 48 10                                  	movdqu %xmm1,0x10(%rax)
  15147f:	f3 0f 7f 00                                     	movdqu %xmm0,(%rax)
  151483:	eb 72                                           	jmp    1514f7 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11c7>
  151485:	0f 28 84 24 e0 02 00 00                         	movaps 0x2e0(%rsp),%xmm0
  15148d:	0f 28 8c 24 f0 02 00 00                         	movaps 0x2f0(%rsp),%xmm1
  151495:	66 0f 6f 94 24 00 03 00 00                      	movdqa 0x300(%rsp),%xmm2
  15149e:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  1514a3:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  1514a8:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
  1514ac:	0f 11 00                                        	movups %xmm0,(%rax)
  1514af:	eb 2c                                           	jmp    1514dd <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11ad>
  1514b1:	48 8b 84 24 80 02 00 00                         	mov    0x280(%rsp),%rax
  1514b9:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
  1514be:	48 89 41 20                                     	mov    %rax,0x20(%rcx)
  1514c2:	0f 28 84 24 60 02 00 00                         	movaps 0x260(%rsp),%xmm0
  1514ca:	0f 28 8c 24 70 02 00 00                         	movaps 0x270(%rsp),%xmm1
  1514d2:	0f 11 49 10                                     	movups %xmm1,0x10(%rcx)
  1514d6:	0f 11 01                                        	movups %xmm0,(%rcx)
  1514d9:	48 89 69 28                                     	mov    %rbp,0x28(%rcx)
  1514dd:	48 8d bc 24 48 02 00 00                         	lea    0x248(%rsp),%rdi
  1514e5:	e8 76 ee f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  1514ea:	48 8d bc 24 30 01 00 00                         	lea    0x130(%rsp),%rdi
  1514f2:	e8 39 f5 f3 ff                                  	call   90a30 <core::ptr::drop_glue::<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>>
  1514f7:	48 8d bc 24 90 02 00 00                         	lea    0x290(%rsp),%rdi
  1514ff:	e8 2c 84 f4 ff                                  	call   99930 <core::ptr::drop_glue::<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>>
  151504:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  15150c:	e8 ff 81 f4 ff                                  	call   99710 <core::ptr::drop_glue::<emuella_j2k_tier1::CodeBlockEncodeScratch>>
  151511:	4c 8b 7c 24 08                                  	mov    0x8(%rsp),%r15
  151516:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
  15151b:	48 8b 5c 24 18                                  	mov    0x18(%rsp),%rbx
  151520:	48 83 7c 24 38 00                               	cmpq   $0x0,0x38(%rsp)
  151526:	0f 85 3d f7 ff ff                               	jne    150c69 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x939>
  15152c:	e9 43 f7 ff ff                                  	jmp    150c74 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x944>
  151531:	48 8d 35 7d 22 ec ff                            	lea    -0x13dd83(%rip),%rsi        # 137b5 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe75>
  151538:	48 8d 7c 24 38                                  	lea    0x38(%rsp),%rdi
  15153d:	ba 02 00 00 00                                  	mov    $0x2,%edx
  151542:	e8 f9 b4 ff ff                                  	call   14ca40 <<alloc::vec::Vec<u8>>::append_elements>
  151547:	48 8b 44 24 48                                  	mov    0x48(%rsp),%rax
  15154c:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
  151551:	48 89 41 18                                     	mov    %rax,0x18(%rcx)
  151555:	f3 0f 6f 44 24 38                               	movdqu 0x38(%rsp),%xmm0
  15155b:	f3 0f 7f 41 08                                  	movdqu %xmm0,0x8(%rcx)
  151560:	48 c7 01 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rcx)
  151567:	48 8d bc 24 90 02 00 00                         	lea    0x290(%rsp),%rdi
  15156f:	e8 bc 83 f4 ff                                  	call   99930 <core::ptr::drop_glue::<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>>
  151574:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  15157c:	e8 8f 81 f4 ff                                  	call   99710 <core::ptr::drop_glue::<emuella_j2k_tier1::CodeBlockEncodeScratch>>
  151581:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
  151587:	74 0b                                           	je     151594 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1264>
  151589:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
  15158e:	ff 15 3c c8 11 00                               	call   *0x11c83c(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  151594:	48 83 bc 24 f8 01 00 00 00                      	cmpq   $0x0,0x1f8(%rsp)
  15159d:	74 0e                                           	je     1515ad <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x127d>
  15159f:	48 8b bc 24 00 02 00 00                         	mov    0x200(%rsp),%rdi
  1515a7:	ff 15 23 c8 11 00                               	call   *0x11c823(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  1515ad:	48 83 7c 24 10 00                               	cmpq   $0x0,0x10(%rsp)
  1515b3:	74 09                                           	je     1515be <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x128e>
  1515b5:	4c 89 ef                                        	mov    %r13,%rdi
  1515b8:	ff 15 12 c8 11 00                               	call   *0x11c812(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  1515be:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
  1515c6:	e8 95 ed f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  1515cb:	4c 8b 7c 24 08                                  	mov    0x8(%rsp),%r15
  1515d0:	e9 45 ef ff ff                                  	jmp    15051a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1ea>
  1515d5:	48 8d 0d 24 5d 11 00                            	lea    0x115d24(%rip),%rcx        # 267300 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x358>
  1515dc:	48 8b 7c 24 50                                  	mov    0x50(%rsp),%rdi
  1515e1:	4c 89 f6                                        	mov    %r14,%rsi
  1515e4:	ff 15 06 ca 11 00                               	call   *0x11ca06(%rip)        # 26dff0 <_DYNAMIC+0x470>
  1515ea:	eb 22                                           	jmp    15160e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12de>
  1515ec:	48 89 ce                                        	mov    %rcx,%rsi
  1515ef:	48 8d 0d da 5c 11 00                            	lea    0x115cda(%rip),%rcx        # 2672d0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x328>
  1515f6:	ff 15 f4 c9 11 00                               	call   *0x11c9f4(%rip)        # 26dff0 <_DYNAMIC+0x470>
  1515fc:	eb 10                                           	jmp    15160e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12de>
  1515fe:	bf 08 00 00 00                                  	mov    $0x8,%edi
  151603:	be 90 00 00 00                                  	mov    $0x90,%esi
  151608:	ff 15 8a c7 11 00                               	call   *0x11c78a(%rip)        # 26dd98 <_DYNAMIC+0x218>
  15160e:	0f 0b                                           	ud2
  151610:	e9 85 00 00 00                                  	jmp    15169a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x136a>
  151615:	49 89 c7                                        	mov    %rax,%r15
  151618:	48 85 ed                                        	test   %rbp,%rbp
  15161b:	0f 84 95 00 00 00                               	je     1516b6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1386>
  151621:	4c 89 f7                                        	mov    %r14,%rdi
  151624:	e9 87 00 00 00                                  	jmp    1516b0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1380>
  151629:	eb 14                                           	jmp    15163f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x130f>
  15162b:	eb 6d                                           	jmp    15169a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x136a>
  15162d:	48 89 5c 24 18                                  	mov    %rbx,0x18(%rsp)
  151632:	48 89 6c 24 10                                  	mov    %rbp,0x10(%rsp)
  151637:	49 89 c7                                        	mov    %rax,%r15
  15163a:	e9 ab 00 00 00                                  	jmp    1516ea <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13ba>
  15163f:	49 89 c7                                        	mov    %rax,%r15
  151642:	eb 72                                           	jmp    1516b6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1386>
  151644:	48 89 6c 24 10                                  	mov    %rbp,0x10(%rsp)
  151649:	49 89 c7                                        	mov    %rax,%r15
  15164c:	e9 bf 00 00 00                                  	jmp    151710 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13e0>
  151651:	48 89 6c 24 10                                  	mov    %rbp,0x10(%rsp)
  151656:	49 89 c7                                        	mov    %rax,%r15
  151659:	e9 cb 00 00 00                                  	jmp    151729 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13f9>
  15165e:	49 89 c7                                        	mov    %rax,%r15
  151661:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
  151669:	e8 f2 ec f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  15166e:	4c 89 ff                                        	mov    %r15,%rdi
  151671:	e8 3a 3e 11 00                                  	call   2654b0 <_Unwind_Resume@plt>
  151676:	49 89 c7                                        	mov    %rax,%r15
  151679:	eb 55                                           	jmp    1516d0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13a0>
  15167b:	49 89 c7                                        	mov    %rax,%r15
  15167e:	48 83 bc 24 18 01 00 00 00                      	cmpq   $0x0,0x118(%rsp)
  151687:	0f 84 d6 f2 ff ff                               	je     150963 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
  15168d:	48 8b bc 24 20 01 00 00                         	mov    0x120(%rsp),%rdi
  151695:	e9 c3 f2 ff ff                                  	jmp    15095d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x62d>
  15169a:	49 89 c7                                        	mov    %rax,%r15
  15169d:	48 83 bc 24 f0 00 00 00 00                      	cmpq   $0x0,0xf0(%rsp)
  1516a6:	74 0e                                           	je     1516b6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1386>
  1516a8:	48 8b bc 24 f8 00 00 00                         	mov    0xf8(%rsp),%rdi
  1516b0:	ff 15 1a c7 11 00                               	call   *0x11c71a(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  1516b6:	48 8d bc 24 48 02 00 00                         	lea    0x248(%rsp),%rdi
  1516be:	e8 9d ec f3 ff                                  	call   90360 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  1516c3:	48 8d bc 24 30 01 00 00                         	lea    0x130(%rsp),%rdi
  1516cb:	e8 60 f3 f3 ff                                  	call   90a30 <core::ptr::drop_glue::<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>>
  1516d0:	48 8d bc 24 90 02 00 00                         	lea    0x290(%rsp),%rdi
  1516d8:	e8 53 82 f4 ff                                  	call   99930 <core::ptr::drop_glue::<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>>
  1516dd:	48 8d bc 24 50 01 00 00                         	lea    0x150(%rsp),%rdi
  1516e5:	e8 26 80 f4 ff                                  	call   99710 <core::ptr::drop_glue::<emuella_j2k_tier1::CodeBlockEncodeScratch>>
  1516ea:	48 83 7c 24 38 00                               	cmpq   $0x0,0x38(%rsp)
  1516f0:	74 0b                                           	je     1516fd <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13cd>
  1516f2:	48 8b 7c 24 40                                  	mov    0x40(%rsp),%rdi
  1516f7:	ff 15 d3 c6 11 00                               	call   *0x11c6d3(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  1516fd:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
  151703:	74 0b                                           	je     151710 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13e0>
  151705:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
  15170a:	ff 15 c0 c6 11 00                               	call   *0x11c6c0(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  151710:	48 83 bc 24 f8 01 00 00 00                      	cmpq   $0x0,0x1f8(%rsp)
  151719:	74 0e                                           	je     151729 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13f9>
  15171b:	48 8b bc 24 00 02 00 00                         	mov    0x200(%rsp),%rdi
  151723:	ff 15 a7 c6 11 00                               	call   *0x11c6a7(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
  151729:	48 83 7c 24 10 00                               	cmpq   $0x0,0x10(%rsp)
  15172f:	0f 84 2e f2 ff ff                               	je     150963 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
  151735:	4c 89 ef                                        	mov    %r13,%rdi
  151738:	e9 20 f2 ff ff                                  	jmp    15095d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x62d>
