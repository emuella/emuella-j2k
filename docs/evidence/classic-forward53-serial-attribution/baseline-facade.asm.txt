Disassembly of section .text:

000000000018d210 <emuella_j2k_core::scalable_lossless::encode_with_limits>:
  18d210:	55                                              	push   %rbp
  18d211:	41 57                                           	push   %r15
  18d213:	41 56                                           	push   %r14
  18d215:	41 55                                           	push   %r13
  18d217:	41 54                                           	push   %r12
  18d219:	53                                              	push   %rbx
  18d21a:	48 81 ec 38 01 00 00                            	sub    $0x138,%rsp
  18d221:	49 89 f4                                        	mov    %rsi,%r12
  18d224:	48 89 fb                                        	mov    %rdi,%rbx
  18d227:	4c 8b 31                                        	mov    (%rcx),%r14
  18d22a:	4c 8b 79 08                                     	mov    0x8(%rcx),%r15
  18d22e:	48 8b 2e                                        	mov    (%rsi),%rbp
  18d231:	4c 8b 6e 08                                     	mov    0x8(%rsi),%r13
  18d235:	48 85 ed                                        	test   %rbp,%rbp
  18d238:	48 89 ee                                        	mov    %rbp,%rsi
  18d23b:	49 0f 44 f5                                     	cmove  %r13,%rsi
  18d23f:	48 8d 7c 24 08                                  	lea    0x8(%rsp),%rdi
  18d244:	4c 89 f1                                        	mov    %r14,%rcx
  18d247:	4d 89 f8                                        	mov    %r15,%r8
  18d24a:	e8 d1 ec fe ff                                  	call   17bf20 <emuella_j2k_core::scalable_lossless::requirements::<false>>
  18d24f:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  18d254:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  18d258:	74 23                                           	je     18d27d <emuella_j2k_core::scalable_lossless::encode_with_limits+0x6d>
  18d25a:	48 8b 4c 24 30                                  	mov    0x30(%rsp),%rcx
  18d25f:	0f 10 44 24 10                                  	movups 0x10(%rsp),%xmm0
  18d264:	0f 10 4c 24 20                                  	movups 0x20(%rsp),%xmm1
  18d269:	48 89 03                                        	mov    %rax,(%rbx)
  18d26c:	0f 11 43 08                                     	movups %xmm0,0x8(%rbx)
  18d270:	0f 11 4b 18                                     	movups %xmm1,0x18(%rbx)
  18d274:	48 89 4b 28                                     	mov    %rcx,0x28(%rbx)
  18d278:	e9 d6 05 00 00                                  	jmp    18d853 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x643>
  18d27d:	48 8d 7c 24 08                                  	lea    0x8(%rsp),%rdi
  18d282:	4c 89 e6                                        	mov    %r12,%rsi
  18d285:	e8 a6 10 ff ff                                  	call   17e330 <emuella_j2k_core::validate_image_view>
  18d28a:	48 83 7c 24 08 ff                               	cmpq   $0xffffffffffffffff,0x8(%rsp)
  18d290:	74 1f                                           	je     18d2b1 <emuella_j2k_core::scalable_lossless::encode_with_limits+0xa1>
  18d292:	0f 10 44 24 08                                  	movups 0x8(%rsp),%xmm0
  18d297:	0f 10 4c 24 18                                  	movups 0x18(%rsp),%xmm1
  18d29c:	0f 10 54 24 28                                  	movups 0x28(%rsp),%xmm2
  18d2a1:	0f 11 53 20                                     	movups %xmm2,0x20(%rbx)
  18d2a5:	0f 11 4b 10                                     	movups %xmm1,0x10(%rbx)
  18d2a9:	0f 11 03                                        	movups %xmm0,(%rbx)
  18d2ac:	e9 a2 05 00 00                                  	jmp    18d853 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x643>
  18d2b1:	48 89 e8                                        	mov    %rbp,%rax
  18d2b4:	48 85 ed                                        	test   %rbp,%rbp
  18d2b7:	0f 84 b7 02 00 00                               	je     18d574 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x364>
  18d2bd:	0f b6 78 0d                                     	movzbl 0xd(%rax),%edi
  18d2c1:	c1 ef 03                                        	shr    $0x3,%edi
  18d2c4:	48 c7 44 24 08 01 00 00 00                      	movq   $0x1,0x8(%rsp)
  18d2cd:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  18d2d0:	0f 11 44 24 10                                  	movups %xmm0,0x10(%rsp)
  18d2d5:	48 89 7c 24 20                                  	mov    %rdi,0x20(%rsp)
  18d2da:	48 c7 44 24 28 01 00 00 00                      	movq   $0x1,0x28(%rsp)
  18d2e3:	0f 11 44 24 30                                  	movups %xmm0,0x30(%rsp)
  18d2e8:	48 89 7c 24 40                                  	mov    %rdi,0x40(%rsp)
  18d2ed:	48 c7 44 24 48 01 00 00 00                      	movq   $0x1,0x48(%rsp)
  18d2f6:	0f 11 44 24 50                                  	movups %xmm0,0x50(%rsp)
  18d2fb:	48 89 7c 24 60                                  	mov    %rdi,0x60(%rsp)
  18d300:	48 c7 44 24 68 01 00 00 00                      	movq   $0x1,0x68(%rsp)
  18d309:	0f 11 44 24 70                                  	movups %xmm0,0x70(%rsp)
  18d30e:	48 89 bc 24 80 00 00 00                         	mov    %rdi,0x80(%rsp)
  18d316:	48 c7 84 24 88 00 00 00 01 00 00 00             	movq   $0x1,0x88(%rsp)
  18d322:	0f 11 84 24 90 00 00 00                         	movups %xmm0,0x90(%rsp)
  18d32a:	48 89 bc 24 a0 00 00 00                         	mov    %rdi,0xa0(%rsp)
  18d332:	48 c7 84 24 a8 00 00 00 01 00 00 00             	movq   $0x1,0xa8(%rsp)
  18d33e:	0f 11 84 24 b0 00 00 00                         	movups %xmm0,0xb0(%rsp)
  18d346:	48 89 bc 24 c0 00 00 00                         	mov    %rdi,0xc0(%rsp)
  18d34e:	48 c7 84 24 c8 00 00 00 01 00 00 00             	movq   $0x1,0xc8(%rsp)
  18d35a:	0f 11 84 24 d0 00 00 00                         	movups %xmm0,0xd0(%rsp)
  18d362:	48 89 bc 24 e0 00 00 00                         	mov    %rdi,0xe0(%rsp)
  18d36a:	48 c7 84 24 e8 00 00 00 01 00 00 00             	movq   $0x1,0xe8(%rsp)
  18d376:	0f 11 84 24 f0 00 00 00                         	movups %xmm0,0xf0(%rsp)
  18d37e:	48 89 bc 24 00 01 00 00                         	mov    %rdi,0x100(%rsp)
  18d386:	0f b7 48 08                                     	movzwl 0x8(%rax),%ecx
  18d38a:	48 85 c9                                        	test   %rcx,%rcx
  18d38d:	0f 84 be 02 00 00                               	je     18d651 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x441>
  18d393:	49 8b 54 24 10                                  	mov    0x10(%r12),%rdx
  18d398:	49 8b 74 24 18                                  	mov    0x18(%r12),%rsi
  18d39d:	48 85 ed                                        	test   %rbp,%rbp
  18d3a0:	0f 84 b3 02 00 00                               	je     18d659 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x449>
  18d3a6:	49 89 c8                                        	mov    %rcx,%r8
  18d3a9:	4c 0f af c7                                     	imul   %rdi,%r8
  18d3ad:	4c 89 6c 24 08                                  	mov    %r13,0x8(%rsp)
  18d3b2:	48 89 54 24 10                                  	mov    %rdx,0x10(%rsp)
  18d3b7:	48 89 74 24 18                                  	mov    %rsi,0x18(%rsp)
  18d3bc:	4c 89 44 24 20                                  	mov    %r8,0x20(%rsp)
  18d3c1:	48 83 f9 01                                     	cmp    $0x1,%rcx
  18d3c5:	0f 84 1c 04 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d3cb:	49 89 d0                                        	mov    %rdx,%r8
  18d3ce:	49 29 f8                                        	sub    %rdi,%r8
  18d3d1:	0f 82 b3 04 00 00                               	jb     18d88a <emuella_j2k_core::scalable_lossless::encode_with_limits+0x67a>
  18d3d7:	4e 8d 0c 2f                                     	lea    (%rdi,%r13,1),%r9
  18d3db:	44 0f b7 50 08                                  	movzwl 0x8(%rax),%r10d
  18d3e0:	4c 0f af d7                                     	imul   %rdi,%r10
  18d3e4:	4c 89 4c 24 28                                  	mov    %r9,0x28(%rsp)
  18d3e9:	4c 89 44 24 30                                  	mov    %r8,0x30(%rsp)
  18d3ee:	48 89 74 24 38                                  	mov    %rsi,0x38(%rsp)
  18d3f3:	4c 89 54 24 40                                  	mov    %r10,0x40(%rsp)
  18d3f8:	83 f9 02                                        	cmp    $0x2,%ecx
  18d3fb:	0f 84 e6 03 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d401:	44 8d 04 3f                                     	lea    (%rdi,%rdi,1),%r8d
  18d405:	49 89 d1                                        	mov    %rdx,%r9
  18d408:	4d 29 c1                                        	sub    %r8,%r9
  18d40b:	0f 82 76 04 00 00                               	jb     18d887 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x677>
  18d411:	4d 01 e8                                        	add    %r13,%r8
  18d414:	44 0f b7 50 08                                  	movzwl 0x8(%rax),%r10d
  18d419:	4c 0f af d7                                     	imul   %rdi,%r10
  18d41d:	4c 89 44 24 48                                  	mov    %r8,0x48(%rsp)
  18d422:	4c 89 4c 24 50                                  	mov    %r9,0x50(%rsp)
  18d427:	48 89 74 24 58                                  	mov    %rsi,0x58(%rsp)
  18d42c:	4c 89 54 24 60                                  	mov    %r10,0x60(%rsp)
  18d431:	83 f9 03                                        	cmp    $0x3,%ecx
  18d434:	0f 84 ad 03 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d43a:	4c 8d 04 7f                                     	lea    (%rdi,%rdi,2),%r8
  18d43e:	49 89 d1                                        	mov    %rdx,%r9
  18d441:	4d 29 c1                                        	sub    %r8,%r9
  18d444:	0f 82 3d 04 00 00                               	jb     18d887 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x677>
  18d44a:	4d 01 e8                                        	add    %r13,%r8
  18d44d:	44 0f b7 50 08                                  	movzwl 0x8(%rax),%r10d
  18d452:	4c 0f af d7                                     	imul   %rdi,%r10
  18d456:	4c 89 44 24 68                                  	mov    %r8,0x68(%rsp)
  18d45b:	4c 89 4c 24 70                                  	mov    %r9,0x70(%rsp)
  18d460:	48 89 74 24 78                                  	mov    %rsi,0x78(%rsp)
  18d465:	4c 89 94 24 80 00 00 00                         	mov    %r10,0x80(%rsp)
  18d46d:	83 f9 04                                        	cmp    $0x4,%ecx
  18d470:	0f 84 71 03 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d476:	44 8d 04 bd 00 00 00 00                         	lea    0x0(,%rdi,4),%r8d
  18d47e:	49 89 d1                                        	mov    %rdx,%r9
  18d481:	4d 29 c1                                        	sub    %r8,%r9
  18d484:	0f 82 fd 03 00 00                               	jb     18d887 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x677>
  18d48a:	4d 01 e8                                        	add    %r13,%r8
  18d48d:	44 0f b7 50 08                                  	movzwl 0x8(%rax),%r10d
  18d492:	4c 0f af d7                                     	imul   %rdi,%r10
  18d496:	4c 89 84 24 88 00 00 00                         	mov    %r8,0x88(%rsp)
  18d49e:	4c 89 8c 24 90 00 00 00                         	mov    %r9,0x90(%rsp)
  18d4a6:	48 89 b4 24 98 00 00 00                         	mov    %rsi,0x98(%rsp)
  18d4ae:	4c 89 94 24 a0 00 00 00                         	mov    %r10,0xa0(%rsp)
  18d4b6:	83 f9 05                                        	cmp    $0x5,%ecx
  18d4b9:	0f 84 28 03 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d4bf:	4c 8d 04 bf                                     	lea    (%rdi,%rdi,4),%r8
  18d4c3:	49 89 d1                                        	mov    %rdx,%r9
  18d4c6:	4d 29 c1                                        	sub    %r8,%r9
  18d4c9:	0f 82 b8 03 00 00                               	jb     18d887 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x677>
  18d4cf:	4d 01 e8                                        	add    %r13,%r8
  18d4d2:	44 0f b7 50 08                                  	movzwl 0x8(%rax),%r10d
  18d4d7:	4c 0f af d7                                     	imul   %rdi,%r10
  18d4db:	4c 89 84 24 a8 00 00 00                         	mov    %r8,0xa8(%rsp)
  18d4e3:	4c 89 8c 24 b0 00 00 00                         	mov    %r9,0xb0(%rsp)
  18d4eb:	48 89 b4 24 b8 00 00 00                         	mov    %rsi,0xb8(%rsp)
  18d4f3:	4c 89 94 24 c0 00 00 00                         	mov    %r10,0xc0(%rsp)
  18d4fb:	83 f9 06                                        	cmp    $0x6,%ecx
  18d4fe:	0f 84 e3 02 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d504:	44 8d 04 3f                                     	lea    (%rdi,%rdi,1),%r8d
  18d508:	4f 8d 04 40                                     	lea    (%r8,%r8,2),%r8
  18d50c:	49 89 d1                                        	mov    %rdx,%r9
  18d50f:	4d 29 c1                                        	sub    %r8,%r9
  18d512:	0f 82 6f 03 00 00                               	jb     18d887 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x677>
  18d518:	4d 01 e8                                        	add    %r13,%r8
  18d51b:	44 0f b7 50 08                                  	movzwl 0x8(%rax),%r10d
  18d520:	4c 0f af d7                                     	imul   %rdi,%r10
  18d524:	4c 89 84 24 c8 00 00 00                         	mov    %r8,0xc8(%rsp)
  18d52c:	4c 89 8c 24 d0 00 00 00                         	mov    %r9,0xd0(%rsp)
  18d534:	48 89 b4 24 d8 00 00 00                         	mov    %rsi,0xd8(%rsp)
  18d53c:	4c 89 94 24 e0 00 00 00                         	mov    %r10,0xe0(%rsp)
  18d544:	83 f9 07                                        	cmp    $0x7,%ecx
  18d547:	0f 84 9a 02 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d54d:	4c 8d 04 fd 00 00 00 00                         	lea    0x0(,%rdi,8),%r8
  18d555:	49 29 f8                                        	sub    %rdi,%r8
  18d558:	48 89 d1                                        	mov    %rdx,%rcx
  18d55b:	4c 29 c1                                        	sub    %r8,%rcx
  18d55e:	0f 82 23 03 00 00                               	jb     18d887 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x677>
  18d564:	4d 01 c5                                        	add    %r8,%r13
  18d567:	0f b7 50 08                                     	movzwl 0x8(%rax),%edx
  18d56b:	48 0f af fa                                     	imul   %rdx,%rdi
  18d56f:	e9 53 02 00 00                                  	jmp    18d7c7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5b7>
  18d574:	49 8b 4c 24 18                                  	mov    0x18(%r12),%rcx
  18d579:	4c 89 e8                                        	mov    %r13,%rax
  18d57c:	48 85 c9                                        	test   %rcx,%rcx
  18d57f:	0f 84 38 fd ff ff                               	je     18d2bd <emuella_j2k_core::scalable_lossless::encode_with_limits+0xad>
  18d585:	49 8b 44 24 10                                  	mov    0x10(%r12),%rax
  18d58a:	48 c1 e1 03                                     	shl    $0x3,%rcx
  18d58e:	48 8d 0c 89                                     	lea    (%rcx,%rcx,4),%rcx
  18d592:	41 8b 55 00                                     	mov    0x0(%r13),%edx
  18d596:	31 f6                                           	xor    %esi,%esi
  18d598:	eb 13                                           	jmp    18d5ad <emuella_j2k_core::scalable_lossless::encode_with_limits+0x39d>
  18d59a:	41 80 f8 02                                     	cmp    $0x2,%r8b
  18d59e:	75 48                                           	jne    18d5e8 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x3d8>
  18d5a0:	48 83 c6 28                                     	add    $0x28,%rsi
  18d5a4:	48 39 f1                                        	cmp    %rsi,%rcx
  18d5a7:	0f 84 bb 02 00 00                               	je     18d868 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x658>
  18d5ad:	39 54 30 18                                     	cmp    %edx,0x18(%rax,%rsi,1)
  18d5b1:	75 35                                           	jne    18d5e8 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x3d8>
  18d5b3:	8b 7c 30 1c                                     	mov    0x1c(%rax,%rsi,1),%edi
  18d5b7:	41 3b 7d 04                                     	cmp    0x4(%r13),%edi
  18d5bb:	75 2b                                           	jne    18d5e8 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x3d8>
  18d5bd:	0f b6 7c 30 22                                  	movzbl 0x22(%rax,%rsi,1),%edi
  18d5c2:	41 3a 7d 0d                                     	cmp    0xd(%r13),%dil
  18d5c6:	75 20                                           	jne    18d5e8 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x3d8>
  18d5c8:	0f b6 7c 30 20                                  	movzbl 0x20(%rax,%rsi,1),%edi
  18d5cd:	41 3a 7d 0b                                     	cmp    0xb(%r13),%dil
  18d5d1:	75 15                                           	jne    18d5e8 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x3d8>
  18d5d3:	0f b6 7c 30 21                                  	movzbl 0x21(%rax,%rsi,1),%edi
  18d5d8:	45 0f b6 45 0c                                  	movzbl 0xc(%r13),%r8d
  18d5dd:	40 80 ff 02                                     	cmp    $0x2,%dil
  18d5e1:	74 b7                                           	je     18d59a <emuella_j2k_core::scalable_lossless::encode_with_limits+0x38a>
  18d5e3:	44 38 c7                                        	cmp    %r8b,%dil
  18d5e6:	74 b8                                           	je     18d5a0 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x390>
  18d5e8:	bf 39 00 00 00                                  	mov    $0x39,%edi
  18d5ed:	ff 15 f5 07 0e 00                               	call   *0xe07f5(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
  18d5f3:	48 85 c0                                        	test   %rax,%rax
  18d5f6:	0f 84 9e 02 00 00                               	je     18d89a <emuella_j2k_core::scalable_lossless::encode_with_limits+0x68a>
  18d5fc:	0f 10 05 96 1e e9 ff                            	movups -0x16e16a(%rip),%xmm0        # 1f499 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x1bf9>
  18d603:	0f 11 40 29                                     	movups %xmm0,0x29(%rax)
  18d607:	0f 10 05 82 1e e9 ff                            	movups -0x16e17e(%rip),%xmm0        # 1f490 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x1bf0>
  18d60e:	0f 11 40 20                                     	movups %xmm0,0x20(%rax)
  18d612:	0f 10 05 67 1e e9 ff                            	movups -0x16e199(%rip),%xmm0        # 1f480 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x1be0>
  18d619:	0f 11 40 10                                     	movups %xmm0,0x10(%rax)
  18d61d:	0f 10 05 4c 1e e9 ff                            	movups -0x16e1b4(%rip),%xmm0        # 1f470 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x1bd0>
  18d624:	0f 11 00                                        	movups %xmm0,(%rax)
  18d627:	48 b9 04 00 00 00 00 00 00 80                   	movabs $0x8000000000000004,%rcx
  18d631:	48 89 0b                                        	mov    %rcx,(%rbx)
  18d634:	48 c7 43 08 39 00 00 00                         	movq   $0x39,0x8(%rbx)
  18d63c:	48 89 43 10                                     	mov    %rax,0x10(%rbx)
  18d640:	48 c7 43 18 39 00 00 00                         	movq   $0x39,0x18(%rbx)
  18d648:	c6 43 20 08                                     	movb   $0x8,0x20(%rbx)
  18d64c:	e9 02 02 00 00                                  	jmp    18d853 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x643>
  18d651:	45 31 c9                                        	xor    %r9d,%r9d
  18d654:	e9 99 01 00 00                                  	jmp    18d7f2 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5e2>
  18d659:	48 85 f6                                        	test   %rsi,%rsi
  18d65c:	0f 84 48 02 00 00                               	je     18d8aa <emuella_j2k_core::scalable_lossless::encode_with_limits+0x69a>
  18d662:	4c 8b 02                                        	mov    (%rdx),%r8
  18d665:	4c 89 44 24 08                                  	mov    %r8,0x8(%rsp)
  18d66a:	0f 10 42 08                                     	movups 0x8(%rdx),%xmm0
  18d66e:	0f 11 44 24 10                                  	movups %xmm0,0x10(%rsp)
  18d673:	48 89 7c 24 20                                  	mov    %rdi,0x20(%rsp)
  18d678:	83 f9 01                                        	cmp    $0x1,%ecx
  18d67b:	0f 84 66 01 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d681:	48 83 fe 01                                     	cmp    $0x1,%rsi
  18d685:	0f 84 2e 02 00 00                               	je     18d8b9 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x6a9>
  18d68b:	4c 8b 42 28                                     	mov    0x28(%rdx),%r8
  18d68f:	4c 89 44 24 28                                  	mov    %r8,0x28(%rsp)
  18d694:	0f 10 42 30                                     	movups 0x30(%rdx),%xmm0
  18d698:	0f 11 44 24 30                                  	movups %xmm0,0x30(%rsp)
  18d69d:	48 89 7c 24 40                                  	mov    %rdi,0x40(%rsp)
  18d6a2:	83 f9 02                                        	cmp    $0x2,%ecx
  18d6a5:	0f 84 3c 01 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d6ab:	48 83 fe 02                                     	cmp    $0x2,%rsi
  18d6af:	0f 84 16 02 00 00                               	je     18d8cb <emuella_j2k_core::scalable_lossless::encode_with_limits+0x6bb>
  18d6b5:	4c 8b 42 50                                     	mov    0x50(%rdx),%r8
  18d6b9:	4c 89 44 24 48                                  	mov    %r8,0x48(%rsp)
  18d6be:	0f 10 42 58                                     	movups 0x58(%rdx),%xmm0
  18d6c2:	0f 11 44 24 50                                  	movups %xmm0,0x50(%rsp)
  18d6c7:	48 89 7c 24 60                                  	mov    %rdi,0x60(%rsp)
  18d6cc:	83 f9 03                                        	cmp    $0x3,%ecx
  18d6cf:	0f 84 12 01 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d6d5:	48 83 fe 03                                     	cmp    $0x3,%rsi
  18d6d9:	0f 84 fe 01 00 00                               	je     18d8dd <emuella_j2k_core::scalable_lossless::encode_with_limits+0x6cd>
  18d6df:	4c 8b 42 78                                     	mov    0x78(%rdx),%r8
  18d6e3:	4c 89 44 24 68                                  	mov    %r8,0x68(%rsp)
  18d6e8:	0f 10 82 80 00 00 00                            	movups 0x80(%rdx),%xmm0
  18d6ef:	0f 11 44 24 70                                  	movups %xmm0,0x70(%rsp)
  18d6f4:	48 89 bc 24 80 00 00 00                         	mov    %rdi,0x80(%rsp)
  18d6fc:	83 f9 04                                        	cmp    $0x4,%ecx
  18d6ff:	0f 84 e2 00 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d705:	48 83 fe 04                                     	cmp    $0x4,%rsi
  18d709:	0f 84 e0 01 00 00                               	je     18d8ef <emuella_j2k_core::scalable_lossless::encode_with_limits+0x6df>
  18d70f:	4c 8b 82 a0 00 00 00                            	mov    0xa0(%rdx),%r8
  18d716:	4c 89 84 24 88 00 00 00                         	mov    %r8,0x88(%rsp)
  18d71e:	0f 10 82 a8 00 00 00                            	movups 0xa8(%rdx),%xmm0
  18d725:	0f 11 84 24 90 00 00 00                         	movups %xmm0,0x90(%rsp)
  18d72d:	48 89 bc 24 a0 00 00 00                         	mov    %rdi,0xa0(%rsp)
  18d735:	83 f9 05                                        	cmp    $0x5,%ecx
  18d738:	0f 84 a9 00 00 00                               	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d73e:	48 83 fe 05                                     	cmp    $0x5,%rsi
  18d742:	0f 84 b9 01 00 00                               	je     18d901 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x6f1>
  18d748:	4c 8b 82 c8 00 00 00                            	mov    0xc8(%rdx),%r8
  18d74f:	4c 89 84 24 a8 00 00 00                         	mov    %r8,0xa8(%rsp)
  18d757:	0f 10 82 d0 00 00 00                            	movups 0xd0(%rdx),%xmm0
  18d75e:	0f 11 84 24 b0 00 00 00                         	movups %xmm0,0xb0(%rsp)
  18d766:	48 89 bc 24 c0 00 00 00                         	mov    %rdi,0xc0(%rsp)
  18d76e:	83 f9 06                                        	cmp    $0x6,%ecx
  18d771:	74 74                                           	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d773:	48 83 fe 06                                     	cmp    $0x6,%rsi
  18d777:	0f 84 96 01 00 00                               	je     18d913 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x703>
  18d77d:	4c 8b 82 f0 00 00 00                            	mov    0xf0(%rdx),%r8
  18d784:	4c 89 84 24 c8 00 00 00                         	mov    %r8,0xc8(%rsp)
  18d78c:	0f 10 82 f8 00 00 00                            	movups 0xf8(%rdx),%xmm0
  18d793:	0f 11 84 24 d0 00 00 00                         	movups %xmm0,0xd0(%rsp)
  18d79b:	48 89 bc 24 e0 00 00 00                         	mov    %rdi,0xe0(%rsp)
  18d7a3:	83 f9 07                                        	cmp    $0x7,%ecx
  18d7a6:	74 3f                                           	je     18d7e7 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x5d7>
  18d7a8:	48 83 fe 07                                     	cmp    $0x7,%rsi
  18d7ac:	0f 84 73 01 00 00                               	je     18d925 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x715>
  18d7b2:	4c 8b aa 18 01 00 00                            	mov    0x118(%rdx),%r13
  18d7b9:	48 8b 8a 20 01 00 00                            	mov    0x120(%rdx),%rcx
  18d7c0:	48 8b b2 28 01 00 00                            	mov    0x128(%rdx),%rsi
  18d7c7:	4c 89 ac 24 e8 00 00 00                         	mov    %r13,0xe8(%rsp)
  18d7cf:	48 89 8c 24 f0 00 00 00                         	mov    %rcx,0xf0(%rsp)
  18d7d7:	48 89 b4 24 f8 00 00 00                         	mov    %rsi,0xf8(%rsp)
  18d7df:	48 89 bc 24 00 01 00 00                         	mov    %rdi,0x100(%rsp)
  18d7e7:	44 0f b7 48 08                                  	movzwl 0x8(%rax),%r9d
  18d7ec:	49 83 f9 09                                     	cmp    $0x9,%r9
  18d7f0:	73 7e                                           	jae    18d870 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x660>
  18d7f2:	8b 30                                           	mov    (%rax),%esi
  18d7f4:	8b 50 04                                        	mov    0x4(%rax),%edx
  18d7f7:	0f b6 48 0d                                     	movzbl 0xd(%rax),%ecx
  18d7fb:	48 8d bc 24 08 01 00 00                         	lea    0x108(%rsp),%rdi
  18d803:	4c 8d 44 24 08                                  	lea    0x8(%rsp),%r8
  18d808:	41 57                                           	push   %r15
  18d80a:	41 56                                           	push   %r14
  18d80c:	ff 15 ce 0d 0e 00                               	call   *0xe0dce(%rip)        # 26e5e0 <_DYNAMIC+0xa60>
  18d812:	48 83 c4 10                                     	add    $0x10,%rsp
  18d816:	48 83 bc 24 08 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x108(%rsp)
  18d81f:	74 13                                           	je     18d834 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x624>
  18d821:	48 8d b4 24 08 01 00 00                         	lea    0x108(%rsp),%rsi
  18d829:	48 89 df                                        	mov    %rbx,%rdi
  18d82c:	ff 15 3e 0e 0e 00                               	call   *0xe0e3e(%rip)        # 26e670 <_DYNAMIC+0xaf0>
  18d832:	eb 1f                                           	jmp    18d853 <emuella_j2k_core::scalable_lossless::encode_with_limits+0x643>
  18d834:	48 8b 84 24 20 01 00 00                         	mov    0x120(%rsp),%rax
  18d83c:	48 89 43 18                                     	mov    %rax,0x18(%rbx)
  18d840:	0f 10 84 24 10 01 00 00                         	movups 0x110(%rsp),%xmm0
  18d848:	0f 11 43 08                                     	movups %xmm0,0x8(%rbx)
  18d84c:	48 c7 03 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rbx)
  18d853:	48 89 d8                                        	mov    %rbx,%rax
  18d856:	48 81 c4 38 01 00 00                            	add    $0x138,%rsp
  18d85d:	5b                                              	pop    %rbx
  18d85e:	41 5c                                           	pop    %r12
  18d860:	41 5d                                           	pop    %r13
  18d862:	41 5e                                           	pop    %r14
  18d864:	41 5f                                           	pop    %r15
  18d866:	5d                                              	pop    %rbp
  18d867:	c3                                              	ret
  18d868:	4c 89 e8                                        	mov    %r13,%rax
  18d86b:	e9 4d fa ff ff                                  	jmp    18d2bd <emuella_j2k_core::scalable_lossless::encode_with_limits+0xad>
  18d870:	48 8d 0d c9 b8 0d 00                            	lea    0xdb8c9(%rip),%rcx        # 269140 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2198>
  18d877:	ba 08 00 00 00                                  	mov    $0x8,%edx
  18d87c:	31 ff                                           	xor    %edi,%edi
  18d87e:	4c 89 ce                                        	mov    %r9,%rsi
  18d881:	ff 15 69 07 0e 00                               	call   *0xe0769(%rip)        # 26dff0 <_DYNAMIC+0x470>
  18d887:	4c 89 c7                                        	mov    %r8,%rdi
  18d88a:	48 8d 0d df b8 0d 00                            	lea    0xdb8df(%rip),%rcx        # 269170 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21c8>
  18d891:	48 89 d6                                        	mov    %rdx,%rsi
  18d894:	ff 15 56 07 0e 00                               	call   *0xe0756(%rip)        # 26dff0 <_DYNAMIC+0x470>
  18d89a:	bf 01 00 00 00                                  	mov    $0x1,%edi
  18d89f:	be 39 00 00 00                                  	mov    $0x39,%esi
  18d8a4:	ff 15 ee 04 0e 00                               	call   *0xe04ee(%rip)        # 26dd98 <_DYNAMIC+0x218>
  18d8aa:	31 ff                                           	xor    %edi,%edi
  18d8ac:	48 8d 15 a5 b8 0d 00                            	lea    0xdb8a5(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d8b3:	ff 15 ef 04 0e 00                               	call   *0xe04ef(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d8b9:	bf 01 00 00 00                                  	mov    $0x1,%edi
  18d8be:	48 8d 15 93 b8 0d 00                            	lea    0xdb893(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d8c5:	ff 15 dd 04 0e 00                               	call   *0xe04dd(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d8cb:	bf 02 00 00 00                                  	mov    $0x2,%edi
  18d8d0:	48 8d 15 81 b8 0d 00                            	lea    0xdb881(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d8d7:	ff 15 cb 04 0e 00                               	call   *0xe04cb(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d8dd:	bf 03 00 00 00                                  	mov    $0x3,%edi
  18d8e2:	48 8d 15 6f b8 0d 00                            	lea    0xdb86f(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d8e9:	ff 15 b9 04 0e 00                               	call   *0xe04b9(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d8ef:	bf 04 00 00 00                                  	mov    $0x4,%edi
  18d8f4:	48 8d 15 5d b8 0d 00                            	lea    0xdb85d(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d8fb:	ff 15 a7 04 0e 00                               	call   *0xe04a7(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d901:	bf 05 00 00 00                                  	mov    $0x5,%edi
  18d906:	48 8d 15 4b b8 0d 00                            	lea    0xdb84b(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d90d:	ff 15 95 04 0e 00                               	call   *0xe0495(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d913:	bf 06 00 00 00                                  	mov    $0x6,%edi
  18d918:	48 8d 15 39 b8 0d 00                            	lea    0xdb839(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d91f:	ff 15 83 04 0e 00                               	call   *0xe0483(%rip)        # 26dda8 <_DYNAMIC+0x228>
  18d925:	bf 07 00 00 00                                  	mov    $0x7,%edi
  18d92a:	48 8d 15 27 b8 0d 00                            	lea    0xdb827(%rip),%rdx        # 269158 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x21b0>
  18d931:	ff 15 71 04 0e 00                               	call   *0xe0471(%rip)        # 26dda8 <_DYNAMIC+0x228>
