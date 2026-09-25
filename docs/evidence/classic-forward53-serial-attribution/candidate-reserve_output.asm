Disassembly of section .text:

00000000001514c0 <emuella_j2k_codestream::scalable_lossless::reserve_output>:
  1514c0:	4c 8b 46 10                                     	mov    0x10(%rsi),%r8
  1514c4:	4c 01 c2                                        	add    %r8,%rdx
  1514c7:	0f 82 21 01 00 00                               	jb     1515ee <emuella_j2k_codestream::scalable_lossless::reserve_output+0x12e>
  1514cd:	48 39 ca                                        	cmp    %rcx,%rdx
  1514d0:	76 33                                           	jbe    151505 <emuella_j2k_codestream::scalable_lossless::reserve_output+0x45>
  1514d2:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  1514dc:	48 89 07                                        	mov    %rax,(%rdi)
  1514df:	48 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%rdi)
  1514e7:	48 8d 05 2e cc ec ff                            	lea    -0x1333d2(%rip),%rax        # 1e11c <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x45c>
  1514ee:	48 89 47 18                                     	mov    %rax,0x18(%rdi)
  1514f2:	48 c7 47 20 32 00 00 00                         	movq   $0x32,0x20(%rdi)
  1514fa:	66 c7 47 28 04 00                               	movw   $0x4,0x28(%rdi)
  151500:	c6 47 2c 0a                                     	movb   $0xa,0x2c(%rdi)
  151504:	c3                                              	ret
  151505:	41 57                                           	push   %r15
  151507:	41 56                                           	push   %r14
  151509:	41 54                                           	push   %r12
  15150b:	53                                              	push   %rbx
  15150c:	48 83 ec 18                                     	sub    $0x18,%rsp
  151510:	48 8b 06                                        	mov    (%rsi),%rax
  151513:	48 39 c2                                        	cmp    %rax,%rdx
  151516:	0f 86 bf 00 00 00                               	jbe    1515db <emuella_j2k_codestream::scalable_lossless::reserve_output+0x11b>
  15151c:	48 8d 1c 00                                     	lea    (%rax,%rax,1),%rbx
  151520:	48 39 d9                                        	cmp    %rbx,%rcx
  151523:	48 0f 42 d9                                     	cmovb  %rcx,%rbx
  151527:	48 39 d3                                        	cmp    %rdx,%rbx
  15152a:	48 0f 46 da                                     	cmovbe %rdx,%rbx
  15152e:	48 89 da                                        	mov    %rbx,%rdx
  151531:	4c 29 c2                                        	sub    %r8,%rdx
  151534:	49 89 c1                                        	mov    %rax,%r9
  151537:	4d 29 c1                                        	sub    %r8,%r9
  15153a:	4c 39 ca                                        	cmp    %r9,%rdx
  15153d:	77 40                                           	ja     15157f <emuella_j2k_codestream::scalable_lossless::reserve_output+0xbf>
  15153f:	48 89 c3                                        	mov    %rax,%rbx
  151542:	48 39 cb                                        	cmp    %rcx,%rbx
  151545:	0f 86 90 00 00 00                               	jbe    1515db <emuella_j2k_codestream::scalable_lossless::reserve_output+0x11b>
  15154b:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  151555:	48 89 07                                        	mov    %rax,(%rdi)
  151558:	48 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%rdi)
  151560:	48 8d 05 80 cb ec ff                            	lea    -0x133480(%rip),%rax        # 1e0e7 <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x427>
  151567:	48 89 47 18                                     	mov    %rax,0x18(%rdi)
  15156b:	48 c7 47 20 35 00 00 00                         	movq   $0x35,0x20(%rdi)
  151573:	66 c7 47 28 04 00                               	movw   $0x4,0x28(%rdi)
  151579:	c6 47 2c 0a                                     	movb   $0xa,0x2c(%rdi)
  15157d:	eb 63                                           	jmp    1515e2 <emuella_j2k_codestream::scalable_lossless::reserve_output+0x122>
  15157f:	49 89 cf                                        	mov    %rcx,%r15
  151582:	49 89 fe                                        	mov    %rdi,%r14
  151585:	49 89 f4                                        	mov    %rsi,%r12
  151588:	48 8b 56 08                                     	mov    0x8(%rsi),%rdx
  15158c:	48 89 e7                                        	mov    %rsp,%rdi
  15158f:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
  151595:	41 b9 01 00 00 00                               	mov    $0x1,%r9d
  15159b:	48 89 c6                                        	mov    %rax,%rsi
  15159e:	48 89 d9                                        	mov    %rbx,%rcx
  1515a1:	e8 6a 96 ff ff                                  	call   14ac10 <<alloc::raw_vec::RawVecInner>::finish_grow>
  1515a6:	83 3c 24 01                                     	cmpl   $0x1,(%rsp)
  1515aa:	75 12                                           	jne    1515be <emuella_j2k_codestream::scalable_lossless::reserve_output+0xfe>
  1515ac:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  1515b6:	48 ff c0                                        	inc    %rax
  1515b9:	49 89 06                                        	mov    %rax,(%r14)
  1515bc:	eb 24                                           	jmp    1515e2 <emuella_j2k_codestream::scalable_lossless::reserve_output+0x122>
  1515be:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  1515c3:	49 89 44 24 08                                  	mov    %rax,0x8(%r12)
  1515c8:	49 89 1c 24                                     	mov    %rbx,(%r12)
  1515cc:	4c 89 f7                                        	mov    %r14,%rdi
  1515cf:	4c 89 f9                                        	mov    %r15,%rcx
  1515d2:	48 39 cb                                        	cmp    %rcx,%rbx
  1515d5:	0f 87 70 ff ff ff                               	ja     15154b <emuella_j2k_codestream::scalable_lossless::reserve_output+0x8b>
  1515db:	48 c7 07 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rdi)
  1515e2:	48 83 c4 18                                     	add    $0x18,%rsp
  1515e6:	5b                                              	pop    %rbx
  1515e7:	41 5c                                           	pop    %r12
  1515e9:	41 5e                                           	pop    %r14
  1515eb:	41 5f                                           	pop    %r15
  1515ed:	c3                                              	ret
  1515ee:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  1515f8:	48 ff c0                                        	inc    %rax
  1515fb:	48 89 07                                        	mov    %rax,(%rdi)
  1515fe:	c3                                              	ret
