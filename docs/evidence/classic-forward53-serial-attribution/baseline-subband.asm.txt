Disassembly of section .text:

00000000000853e0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>>:
   853e0:	55                                              	push   %rbp
   853e1:	41 57                                           	push   %r15
   853e3:	41 56                                           	push   %r14
   853e5:	41 55                                           	push   %r13
   853e7:	41 54                                           	push   %r12
   853e9:	53                                              	push   %rbx
   853ea:	48 81 ec 78 01 00 00                            	sub    $0x178,%rsp
   853f1:	4d 89 c6                                        	mov    %r8,%r14
   853f4:	4d 8b 00                                        	mov    (%r8),%r8
   853f7:	45 8b 56 18                                     	mov    0x18(%r14),%r10d
   853fb:	4c 89 c3                                        	mov    %r8,%rbx
   853fe:	4c 01 d3                                        	add    %r10,%rbx
   85401:	0f 82 96 00 00 00                               	jb     8549d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   85407:	89 f5                                           	mov    %esi,%ebp
   85409:	4d 8b 5e 08                                     	mov    0x8(%r14),%r11
   8540d:	41 8b 76 1c                                     	mov    0x1c(%r14),%esi
   85411:	4d 89 dc                                        	mov    %r11,%r12
   85414:	49 01 f4                                        	add    %rsi,%r12
   85417:	0f 82 80 00 00 00                               	jb     8549d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   8541d:	4d 89 c5                                        	mov    %r8,%r13
   85420:	49 c1 ed 06                                     	shr    $0x6,%r13
   85424:	49 89 df                                        	mov    %rbx,%r15
   85427:	49 c1 ef 06                                     	shr    $0x6,%r15
   8542b:	89 d8                                           	mov    %ebx,%eax
   8542d:	83 e0 3f                                        	and    $0x3f,%eax
   85430:	48 83 f8 01                                     	cmp    $0x1,%rax
   85434:	49 83 df ff                                     	sbb    $0xffffffffffffffff,%r15
   85438:	4c 89 f8                                        	mov    %r15,%rax
   8543b:	4c 29 e8                                        	sub    %r13,%rax
   8543e:	72 5d                                           	jb     8549d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   85440:	48 3d ff ff 00 00                               	cmp    $0xffff,%rax
   85446:	77 55                                           	ja     8549d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   85448:	48 89 94 24 68 01 00 00                         	mov    %rdx,0x168(%rsp)
   85450:	48 89 8c 24 70 01 00 00                         	mov    %rcx,0x170(%rsp)
   85458:	48 89 44 24 30                                  	mov    %rax,0x30(%rsp)
   8545d:	44 89 4c 24 24                                  	mov    %r9d,0x24(%rsp)
   85462:	4c 89 44 24 18                                  	mov    %r8,0x18(%rsp)
   85467:	4c 89 9c 24 b8 00 00 00                         	mov    %r11,0xb8(%rsp)
   8546f:	49 c1 eb 06                                     	shr    $0x6,%r11
   85473:	4c 89 e0                                        	mov    %r12,%rax
   85476:	49 c1 ec 06                                     	shr    $0x6,%r12
   8547a:	48 89 84 24 60 01 00 00                         	mov    %rax,0x160(%rsp)
   85482:	83 e0 3f                                        	and    $0x3f,%eax
   85485:	48 83 f8 01                                     	cmp    $0x1,%rax
   85489:	49 83 dc ff                                     	sbb    $0xffffffffffffffff,%r12
   8548d:	4c 89 e0                                        	mov    %r12,%rax
   85490:	4c 29 d8                                        	sub    %r11,%rax
   85493:	72 08                                           	jb     8549d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   85495:	48 3d ff ff 00 00                               	cmp    $0xffff,%rax
   8549b:	76 27                                           	jbe    854c4 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xe4>
   8549d:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   854a7:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
   854ab:	48 c7 07 01 00 00 00                            	movq   $0x1,(%rdi)
   854b2:	48 81 c4 78 01 00 00                            	add    $0x178,%rsp
   854b9:	5b                                              	pop    %rbx
   854ba:	41 5c                                           	pop    %r12
   854bc:	41 5d                                           	pop    %r13
   854be:	41 5e                                           	pop    %r14
   854c0:	41 5f                                           	pop    %r15
   854c2:	5d                                              	pop    %rbp
   854c3:	c3                                              	ret
   854c4:	48 89 c2                                        	mov    %rax,%rdx
   854c7:	48 0f af 54 24 30                               	imul   0x30(%rsp),%rdx
   854cd:	48 85 d2                                        	test   %rdx,%rdx
   854d0:	48 89 7c 24 10                                  	mov    %rdi,0x10(%rsp)
   854d5:	4c 89 9c 24 a8 00 00 00                         	mov    %r11,0xa8(%rsp)
   854dd:	4c 89 94 24 40 01 00 00                         	mov    %r10,0x140(%rsp)
   854e5:	48 89 b4 24 38 01 00 00                         	mov    %rsi,0x138(%rsp)
   854ed:	48 89 84 24 90 00 00 00                         	mov    %rax,0x90(%rsp)
   854f5:	74 32                                           	je     85529 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x149>
   854f7:	48 89 54 24 50                                  	mov    %rdx,0x50(%rsp)
   854fc:	48 89 d7                                        	mov    %rdx,%rdi
   854ff:	48 c1 e7 05                                     	shl    $0x5,%rdi
   85503:	48 89 7c 24 48                                  	mov    %rdi,0x48(%rsp)
   85508:	ff 15 da 88 1e 00                               	call   *0x1e88da(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   8550e:	48 85 c0                                        	test   %rax,%rax
   85511:	0f 84 a6 08 00 00                               	je     85dbd <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9dd>
   85517:	48 89 c1                                        	mov    %rax,%rcx
   8551a:	4c 8b 9c 24 a8 00 00 00                         	mov    0xa8(%rsp),%r11
   85522:	48 8b 54 24 50                                  	mov    0x50(%rsp),%rdx
   85527:	eb 05                                           	jmp    8552e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x14e>
   85529:	b9 08 00 00 00                                  	mov    $0x8,%ecx
   8552e:	48 89 54 24 70                                  	mov    %rdx,0x70(%rsp)
   85533:	48 89 4c 24 78                                  	mov    %rcx,0x78(%rsp)
   85538:	48 c7 84 24 80 00 00 00 00 00 00 00             	movq   $0x0,0x80(%rsp)
   85544:	41 8b 46 10                                     	mov    0x10(%r14),%eax
   85548:	48 89 44 24 48                                  	mov    %rax,0x48(%rsp)
   8554d:	41 8b 46 14                                     	mov    0x14(%r14),%eax
   85551:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
   85559:	48 c7 44 24 58 00 00 00 00                      	movq   $0x0,0x58(%rsp)
   85562:	48 c7 44 24 60 01 00 00 00                      	movq   $0x1,0x60(%rsp)
   8556b:	48 c7 44 24 68 00 00 00 00                      	movq   $0x0,0x68(%rsp)
   85574:	4d 39 dc                                        	cmp    %r11,%r12
   85577:	75 0d                                           	jne    85586 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x1a6>
   85579:	45 0f b6 4e 22                                  	movzbl 0x22(%r14),%r9d
   8557e:	b0 01                                           	mov    $0x1,%al
   85580:	8b 7c 24 24                                     	mov    0x24(%rsp),%edi
   85584:	eb 33                                           	jmp    855b9 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x1d9>
   85586:	48 83 bc 24 c0 01 00 00 00                      	cmpq   $0x0,0x1c0(%rsp)
   8558f:	48 8d 44 24 58                                  	lea    0x58(%rsp),%rax
   85594:	48 0f 44 84 24 b0 01 00 00                      	cmove  0x1b0(%rsp),%rax
   8559d:	48 89 84 24 30 01 00 00                         	mov    %rax,0x130(%rsp)
   855a5:	45 0f b6 4e 22                                  	movzbl 0x22(%r14),%r9d
   855aa:	b0 01                                           	mov    $0x1,%al
   855ac:	4d 39 ef                                        	cmp    %r13,%r15
   855af:	8b 7c 24 24                                     	mov    0x24(%rsp),%edi
   855b3:	0f 85 83 00 00 00                               	jne    8563c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x25c>
   855b9:	4c 8b 44 24 30                                  	mov    0x30(%rsp),%r8
   855be:	41 0f b7 4e 20                                  	movzwl 0x20(%r14),%ecx
   855c3:	0f 10 44 24 70                                  	movups 0x70(%rsp),%xmm0
   855c8:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
   855cd:	0f 11 46 08                                     	movups %xmm0,0x8(%rsi)
   855d1:	48 8b 94 24 80 00 00 00                         	mov    0x80(%rsp),%rdx
   855d9:	48 89 56 18                                     	mov    %rdx,0x18(%rsi)
   855dd:	48 8b 54 24 48                                  	mov    0x48(%rsp),%rdx
   855e2:	89 56 20                                        	mov    %edx,0x20(%rsi)
   855e5:	48 8b 94 24 a0 00 00 00                         	mov    0xa0(%rsp),%rdx
   855ed:	89 56 24                                        	mov    %edx,0x24(%rsi)
   855f0:	48 8b 94 24 40 01 00 00                         	mov    0x140(%rsp),%rdx
   855f8:	89 56 28                                        	mov    %edx,0x28(%rsi)
   855fb:	48 8b 94 24 38 01 00 00                         	mov    0x138(%rsp),%rdx
   85603:	89 56 2c                                        	mov    %edx,0x2c(%rsi)
   85606:	66 44 89 46 30                                  	mov    %r8w,0x30(%rsi)
   8560b:	48 8b 94 24 90 00 00 00                         	mov    0x90(%rsp),%rdx
   85613:	66 89 56 32                                     	mov    %dx,0x32(%rsi)
   85617:	44 88 4e 34                                     	mov    %r9b,0x34(%rsi)
   8561b:	66 89 4e 35                                     	mov    %cx,0x35(%rsi)
   8561f:	40 88 7e 37                                     	mov    %dil,0x37(%rsi)
   85623:	48 c7 06 00 00 00 00                            	movq   $0x0,(%rsi)
   8562a:	84 c0                                           	test   %al,%al
   8562c:	0f 85 80 fe ff ff                               	jne    854b2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xd2>
   85632:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
   85637:	e9 6b 07 00 00                                  	jmp    85da7 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9c7>
   8563c:	48 89 8c 24 98 00 00 00                         	mov    %rcx,0x98(%rsp)
   85644:	41 89 e8                                        	mov    %ebp,%r8d
   85647:	40 0f b6 c7                                     	movzbl %dil,%eax
   8564b:	48 c1 e0 20                                     	shl    $0x20,%rax
   8564f:	48 89 84 24 e8 00 00 00                         	mov    %rax,0xe8(%rsp)
   85657:	44 88 4c 24 0f                                  	mov    %r9b,0xf(%rsp)
   8565c:	41 0f b6 c1                                     	movzbl %r9b,%eax
   85660:	48 89 84 24 08 01 00 00                         	mov    %rax,0x108(%rsp)
   85668:	49 ba 00 00 00 00 00 00 00 04                   	movabs $0x400000000000000,%r10
   85672:	4c 89 d0                                        	mov    %r10,%rax
   85675:	4c 29 e8                                        	sub    %r13,%rax
   85678:	48 89 84 24 28 01 00 00                         	mov    %rax,0x128(%rsp)
   85680:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   85685:	48 f7 d0                                        	not    %rax
   85688:	48 c1 e8 06                                     	shr    $0x6,%rax
   8568c:	48 89 84 24 20 01 00 00                         	mov    %rax,0x120(%rsp)
   85694:	4d 29 da                                        	sub    %r11,%r10
   85697:	48 8b 84 24 b8 00 00 00                         	mov    0xb8(%rsp),%rax
   8569f:	48 89 c1                                        	mov    %rax,%rcx
   856a2:	48 f7 d1                                        	not    %rcx
   856a5:	48 c1 e9 06                                     	shr    $0x6,%rcx
   856a9:	48 89 8c 24 f0 00 00 00                         	mov    %rcx,0xf0(%rsp)
   856b1:	49 c1 e5 06                                     	shl    $0x6,%r13
   856b5:	45 31 ff                                        	xor    %r15d,%r15d
   856b8:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   856c1:	4c 89 84 24 88 00 00 00                         	mov    %r8,0x88(%rsp)
   856c9:	4c 89 94 24 d8 00 00 00                         	mov    %r10,0xd8(%rsp)
   856d1:	4f 8d 0c 3b                                     	lea    (%r11,%r15,1),%r9
   856d5:	49 c1 e1 06                                     	shl    $0x6,%r9
   856d9:	4c 39 c8                                        	cmp    %r9,%rax
   856dc:	4c 89 ce                                        	mov    %r9,%rsi
   856df:	48 0f 47 f0                                     	cmova  %rax,%rsi
   856e3:	49 83 c1 40                                     	add    $0x40,%r9
   856e7:	48 8b 8c 24 60 01 00 00                         	mov    0x160(%rsp),%rcx
   856ef:	4c 39 c9                                        	cmp    %r9,%rcx
   856f2:	4c 0f 42 c9                                     	cmovb  %rcx,%r9
   856f6:	48 89 f1                                        	mov    %rsi,%rcx
   856f9:	48 29 c1                                        	sub    %rax,%rcx
   856fc:	48 8b bc 24 a0 00 00 00                         	mov    0xa0(%rsp),%rdi
   85704:	48 01 f9                                        	add    %rdi,%rcx
   85707:	48 89 c8                                        	mov    %rcx,%rax
   8570a:	49 f7 e0                                        	mul    %r8
   8570d:	48 89 84 24 18 01 00 00                         	mov    %rax,0x118(%rsp)
   85715:	0f 90 c0                                        	seto   %al
   85718:	4d 39 d7                                        	cmp    %r10,%r15
   8571b:	0f 84 32 05 00 00                               	je     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85721:	4c 3b bc 24 f0 00 00 00                         	cmp    0xf0(%rsp),%r15
   85729:	0f 84 83 06 00 00                               	je     85db2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9d2>
   8572f:	49 39 f1                                        	cmp    %rsi,%r9
   85732:	0f 82 eb 04 00 00                               	jb     85c23 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x843>
   85738:	49 29 f1                                        	sub    %rsi,%r9
   8573b:	49 81 f9 ff ff 00 00                            	cmp    $0xffff,%r9
   85742:	0f 87 db 04 00 00                               	ja     85c23 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x843>
   85748:	48 39 f9                                        	cmp    %rdi,%rcx
   8574b:	4c 89 4c 24 38                                  	mov    %r9,0x38(%rsp)
   85750:	0f 82 8a 05 00 00                               	jb     85ce0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x900>
   85756:	84 c0                                           	test   %al,%al
   85758:	0f 85 82 05 00 00                               	jne    85ce0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x900>
   8575e:	4c 89 bc 24 00 01 00 00                         	mov    %r15,0x100(%rsp)
   85766:	49 8d 47 01                                     	lea    0x1(%r15),%rax
   8576a:	48 89 84 24 e0 00 00 00                         	mov    %rax,0xe0(%rsp)
   85772:	49 c1 e1 10                                     	shl    $0x10,%r9
   85776:	4c 03 8c 24 e8 00 00 00                         	add    0xe8(%rsp),%r9
   8577e:	4c 89 8c 24 10 01 00 00                         	mov    %r9,0x110(%rsp)
   85786:	4c 8b 64 24 40                                  	mov    0x40(%rsp),%r12
   8578b:	49 c1 e4 05                                     	shl    $0x5,%r12
   8578f:	4c 89 ee                                        	mov    %r13,%rsi
   85792:	31 d2                                           	xor    %edx,%edx
   85794:	e9 9e 00 00 00                                  	jmp    85837 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x457>
   85799:	4c 8b 44 24 50                                  	mov    0x50(%rsp),%r8
   8579e:	49 8d 40 01                                     	lea    0x1(%r8),%rax
   857a2:	44 89 e1                                        	mov    %r12d,%ecx
   857a5:	41 c1 ec 18                                     	shr    $0x18,%r12d
   857a9:	41 80 e4 01                                     	and    $0x1,%r12b
   857ad:	48 8b 94 24 98 00 00 00                         	mov    0x98(%rsp),%rdx
   857b5:	48 8b b4 24 50 01 00 00                         	mov    0x150(%rsp),%rsi
   857bd:	48 89 34 3a                                     	mov    %rsi,(%rdx,%rdi,1)
   857c1:	48 8b b4 24 48 01 00 00                         	mov    0x148(%rsp),%rsi
   857c9:	48 89 74 3a 08                                  	mov    %rsi,0x8(%rdx,%rdi,1)
   857ce:	66 44 89 44 3a 10                               	mov    %r8w,0x10(%rdx,%rdi,1)
   857d4:	48 8b b4 24 00 01 00 00                         	mov    0x100(%rsp),%rsi
   857dc:	66 89 74 3a 12                                  	mov    %si,0x12(%rdx,%rdi,1)
   857e1:	66 89 6c 3a 14                                  	mov    %bp,0x14(%rdx,%rdi,1)
   857e6:	48 8b 74 24 38                                  	mov    0x38(%rsp),%rsi
   857eb:	66 89 74 3a 16                                  	mov    %si,0x16(%rdx,%rdi,1)
   857f0:	66 89 4c 3a 18                                  	mov    %cx,0x18(%rdx,%rdi,1)
   857f5:	44 88 7c 3a 1a                                  	mov    %r15b,0x1a(%rdx,%rdi,1)
   857fa:	44 88 64 3a 1b                                  	mov    %r12b,0x1b(%rdx,%rdi,1)
   857ff:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
   85804:	4c 01 c1                                        	add    %r8,%rcx
   85807:	48 ff c1                                        	inc    %rcx
   8580a:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
   85812:	48 83 c7 20                                     	add    $0x20,%rdi
   85816:	48 89 c2                                        	mov    %rax,%rdx
   85819:	48 39 44 24 30                                  	cmp    %rax,0x30(%rsp)
   8581e:	4c 8b 9c 24 a8 00 00 00                         	mov    0xa8(%rsp),%r11
   85826:	49 89 fc                                        	mov    %rdi,%r12
   85829:	48 8b b4 24 58 01 00 00                         	mov    0x158(%rsp),%rsi
   85831:	0f 84 23 03 00 00                               	je     85b5a <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x77a>
   85837:	48 39 94 24 28 01 00 00                         	cmp    %rdx,0x128(%rsp)
   8583f:	0f 84 0e 04 00 00                               	je     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85845:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
   8584a:	48 39 f1                                        	cmp    %rsi,%rcx
   8584d:	48 89 f0                                        	mov    %rsi,%rax
   85850:	48 0f 47 c1                                     	cmova  %rcx,%rax
   85854:	48 89 54 24 50                                  	mov    %rdx,0x50(%rsp)
   85859:	48 39 94 24 20 01 00 00                         	cmp    %rdx,0x120(%rsp)
   85861:	4c 8b 8c 24 88 00 00 00                         	mov    0x88(%rsp),%r9
   85869:	0f 84 e4 03 00 00                               	je     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   8586f:	48 83 c6 40                                     	add    $0x40,%rsi
   85873:	48 39 f3                                        	cmp    %rsi,%rbx
   85876:	48 89 f5                                        	mov    %rsi,%rbp
   85879:	48 0f 42 eb                                     	cmovb  %rbx,%rbp
   8587d:	48 29 c5                                        	sub    %rax,%rbp
   85880:	0f 82 cd 03 00 00                               	jb     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85886:	48 81 fd ff ff 00 00                            	cmp    $0xffff,%rbp
   8588d:	0f 87 c0 03 00 00                               	ja     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85893:	48 89 e9                                        	mov    %rbp,%rcx
   85896:	48 0f af 4c 24 38                               	imul   0x38(%rsp),%rcx
   8589c:	48 81 c1 ff ef ff ff                            	add    $0xffffffffffffefff,%rcx
   858a3:	48 81 f9 00 f0 ff ff                            	cmp    $0xfffffffffffff000,%rcx
   858aa:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   858af:	0f 82 81 04 00 00                               	jb     85d36 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x956>
   858b5:	48 2b 44 24 18                                  	sub    0x18(%rsp),%rax
   858ba:	48 03 44 24 48                                  	add    0x48(%rsp),%rax
   858bf:	0f 82 a9 04 00 00                               	jb     85d6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   858c5:	48 03 84 24 18 01 00 00                         	add    0x118(%rsp),%rax
   858cd:	0f 82 9b 04 00 00                               	jb     85d6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   858d3:	4c 89 a4 24 b0 00 00 00                         	mov    %r12,0xb0(%rsp)
   858db:	48 8b 94 24 70 01 00 00                         	mov    0x170(%rsp),%rdx
   858e3:	48 29 c2                                        	sub    %rax,%rdx
   858e6:	0f 82 82 04 00 00                               	jb     85d6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   858ec:	48 89 b4 24 58 01 00 00                         	mov    %rsi,0x158(%rsp)
   858f4:	48 8b 8c 24 68 01 00 00                         	mov    0x168(%rsp),%rcx
   858fc:	48 8d 34 81                                     	lea    (%rcx,%rax,4),%rsi
   85900:	48 8b 84 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rax
   85908:	48 8b 40 10                                     	mov    0x10(%rax),%rax
   8590c:	48 89 84 24 50 01 00 00                         	mov    %rax,0x150(%rsp)
   85914:	48 c7 44 24 68 00 00 00 00                      	movq   $0x0,0x68(%rsp)
   8591d:	48 8b 84 24 08 01 00 00                         	mov    0x108(%rsp),%rax
   85925:	48 8d 0d 54 bc f8 ff                            	lea    -0x743ac(%rip),%rcx        # 11580 <anon.02ffda8f51e6076a26652e0a4fd6fa8d.1589.llvm.5406940975955002095+0x20>
   8592c:	4c 8b 04 c1                                     	mov    (%rcx,%rax,8),%r8
   85930:	4c 0b 84 24 10 01 00 00                         	or     0x110(%rsp),%r8
   85938:	49 09 e8                                        	or     %rbp,%r8
   8593b:	48 8b 84 24 b8 01 00 00                         	mov    0x1b8(%rsp),%rax
   85943:	48 89 04 24                                     	mov    %rax,(%rsp)
   85947:	48 8d bc 24 c0 00 00 00                         	lea    0xc0(%rsp),%rdi
   8594f:	4c 89 c9                                        	mov    %r9,%rcx
   85952:	4c 8b 8c 24 30 01 00 00                         	mov    0x130(%rsp),%r9
   8595a:	ff 15 f8 88 1e 00                               	call   *0x1e88f8(%rip)        # 26e258 <_DYNAMIC+0x6d8>
   85960:	0f b6 84 24 c0 00 00 00                         	movzbl 0xc0(%rsp),%eax
   85968:	4c 8b bc 24 d0 00 00 00                         	mov    0xd0(%rsp),%r15
   85970:	3c ff                                           	cmp    $0xff,%al
   85972:	0f 85 35 02 00 00                               	jne    85bad <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x7cd>
   85978:	48 8b 84 24 c8 00 00 00                         	mov    0xc8(%rsp),%rax
   85980:	48 89 84 24 48 01 00 00                         	mov    %rax,0x148(%rsp)
   85988:	48 8b 84 24 c0 01 00 00                         	mov    0x1c0(%rsp),%rax
   85990:	a8 01                                           	test   $0x1,%al
   85992:	0f 84 d4 00 00 00                               	je     85a6c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x68c>
   85998:	48 8b 54 24 68                                  	mov    0x68(%rsp),%rdx
   8599d:	48 8b 8c 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rcx
   859a5:	4c 8b 61 10                                     	mov    0x10(%rcx),%r12
   859a9:	49 8d 04 14                                     	lea    (%r12,%rdx,1),%rax
   859ad:	48 3b 84 24 c8 01 00 00                         	cmp    0x1c8(%rsp),%rax
   859b5:	0f 87 5a 02 00 00                               	ja     85c15 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x835>
   859bb:	48 8b 31                                        	mov    (%rcx),%rsi
   859be:	48 39 f0                                        	cmp    %rsi,%rax
   859c1:	76 50                                           	jbe    85a13 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x633>
   859c3:	4c 8d 14 36                                     	lea    (%rsi,%rsi,1),%r10
   859c7:	48 8b 8c 24 c8 01 00 00                         	mov    0x1c8(%rsp),%rcx
   859cf:	4c 39 d1                                        	cmp    %r10,%rcx
   859d2:	4c 0f 42 d1                                     	cmovb  %rcx,%r10
   859d6:	49 39 c2                                        	cmp    %rax,%r10
   859d9:	4c 0f 46 d0                                     	cmovbe %rax,%r10
   859dd:	4c 89 d0                                        	mov    %r10,%rax
   859e0:	4c 29 e0                                        	sub    %r12,%rax
   859e3:	48 89 f1                                        	mov    %rsi,%rcx
   859e6:	4c 29 e1                                        	sub    %r12,%rcx
   859e9:	48 39 c8                                        	cmp    %rcx,%rax
   859ec:	0f 87 cf 00 00 00                               	ja     85ac1 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x6e1>
   859f2:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   859fa:	48 3b b4 24 c8 01 00 00                         	cmp    0x1c8(%rsp),%rsi
   85a02:	0f 87 5f 02 00 00                               	ja     85c67 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x887>
   85a08:	48 8b 54 24 68                                  	mov    0x68(%rsp),%rdx
   85a0d:	4c 8b 67 10                                     	mov    0x10(%rdi),%r12
   85a11:	eb 08                                           	jmp    85a1b <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x63b>
   85a13:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   85a1b:	48 8b 44 24 60                                  	mov    0x60(%rsp),%rax
   85a20:	48 89 84 24 f8 00 00 00                         	mov    %rax,0xf8(%rsp)
   85a28:	4c 29 e6                                        	sub    %r12,%rsi
   85a2b:	48 39 f2                                        	cmp    %rsi,%rdx
   85a2e:	0f 87 f8 00 00 00                               	ja     85b2c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x74c>
   85a34:	48 85 d2                                        	test   %rdx,%rdx
   85a37:	74 2c                                           	je     85a65 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x685>
   85a39:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
   85a3e:	48 8b 7f 08                                     	mov    0x8(%rdi),%rdi
   85a42:	4c 01 e7                                        	add    %r12,%rdi
   85a45:	48 8b b4 24 f8 00 00 00                         	mov    0xf8(%rsp),%rsi
   85a4d:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
   85a52:	ff 15 60 83 1e 00                               	call   *0x1e8360(%rip)        # 26ddb8 <memcpy@GLIBC_2.14>
   85a58:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
   85a5d:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   85a65:	49 01 d4                                        	add    %rdx,%r12
   85a68:	4c 89 67 10                                     	mov    %r12,0x10(%rdi)
   85a6c:	4d 89 fc                                        	mov    %r15,%r12
   85a6f:	41 c1 ef 10                                     	shr    $0x10,%r15d
   85a73:	41 fe c7                                        	inc    %r15b
   85a76:	0f 84 2f 02 00 00                               	je     85cab <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x8cb>
   85a7c:	48 8b 44 24 40                                  	mov    0x40(%rsp),%rax
   85a81:	48 8b 4c 24 50                                  	mov    0x50(%rsp),%rcx
   85a86:	48 01 c8                                        	add    %rcx,%rax
   85a89:	48 3b 44 24 70                                  	cmp    0x70(%rsp),%rax
   85a8e:	48 8b bc 24 b0 00 00 00                         	mov    0xb0(%rsp),%rdi
   85a96:	0f 85 fd fc ff ff                               	jne    85799 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x3b9>
   85a9c:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
   85aa1:	ff 15 b9 87 1e 00                               	call   *0x1e87b9(%rip)        # 26e260 <_DYNAMIC+0x6e0>
   85aa7:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   85aac:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   85ab4:	48 8b bc 24 b0 00 00 00                         	mov    0xb0(%rsp),%rdi
   85abc:	e9 d8 fc ff ff                                  	jmp    85799 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x3b9>
   85ac1:	4c 89 7c 24 28                                  	mov    %r15,0x28(%rsp)
   85ac6:	4c 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%r15
   85ace:	49 8b 57 08                                     	mov    0x8(%r15),%rdx
   85ad2:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   85ad8:	41 b9 01 00 00 00                               	mov    $0x1,%r9d
   85ade:	48 8d bc 24 c0 00 00 00                         	lea    0xc0(%rsp),%rdi
   85ae6:	4c 89 d1                                        	mov    %r10,%rcx
   85ae9:	4d 89 d4                                        	mov    %r10,%r12
   85aec:	e8 af 3e 0c 00                                  	call   1499a0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   85af1:	80 bc 24 c0 00 00 00 00                         	cmpb   $0x0,0xc0(%rsp)
   85af9:	0f 85 d5 01 00 00                               	jne    85cd4 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x8f4>
   85aff:	48 8b 84 24 c8 00 00 00                         	mov    0xc8(%rsp),%rax
   85b07:	49 89 47 08                                     	mov    %rax,0x8(%r15)
   85b0b:	4d 89 27                                        	mov    %r12,(%r15)
   85b0e:	4c 89 ff                                        	mov    %r15,%rdi
   85b11:	4c 8b 7c 24 28                                  	mov    0x28(%rsp),%r15
   85b16:	4c 89 e6                                        	mov    %r12,%rsi
   85b19:	48 3b b4 24 c8 01 00 00                         	cmp    0x1c8(%rsp),%rsi
   85b21:	0f 86 e1 fe ff ff                               	jbe    85a08 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x628>
   85b27:	e9 3b 01 00 00                                  	jmp    85c67 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x887>
   85b2c:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   85b31:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   85b37:	4c 89 e6                                        	mov    %r12,%rsi
   85b3a:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
   85b3f:	e8 ac 49 02 00                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   85b44:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   85b4c:	4c 8b 67 10                                     	mov    0x10(%rdi),%r12
   85b50:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
   85b55:	e9 df fe ff ff                                  	jmp    85a39 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x659>
   85b5a:	48 01 44 24 40                                  	add    %rax,0x40(%rsp)
   85b5f:	48 8b 84 24 e0 00 00 00                         	mov    0xe0(%rsp),%rax
   85b67:	49 89 c7                                        	mov    %rax,%r15
   85b6a:	48 3b 84 24 90 00 00 00                         	cmp    0x90(%rsp),%rax
   85b72:	48 8b 84 24 b8 00 00 00                         	mov    0xb8(%rsp),%rax
   85b7a:	4c 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%r8
   85b82:	4c 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%r10
   85b8a:	0f 85 41 fb ff ff                               	jne    856d1 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x2f1>
   85b90:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
   85b96:	0f 94 c0                                        	sete   %al
   85b99:	8b 7c 24 24                                     	mov    0x24(%rsp),%edi
   85b9d:	4c 8b 44 24 30                                  	mov    0x30(%rsp),%r8
   85ba2:	44 0f b6 4c 24 0f                               	movzbl 0xf(%rsp),%r9d
   85ba8:	e9 11 fa ff ff                                  	jmp    855be <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x1de>
   85bad:	48 b9 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rcx
   85bb7:	48 8d 51 fe                                     	lea    -0x2(%rcx),%rdx
   85bbb:	48 ff c9                                        	dec    %rcx
   85bbe:	3c 03                                           	cmp    $0x3,%al
   85bc0:	48 0f 44 ca                                     	cmove  %rdx,%rcx
   85bc4:	48 8d 05 93 d9 f8 ff                            	lea    -0x7266d(%rip),%rax        # 1355e <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xc1e>
   85bcb:	48 0f 44 84 24 c8 00 00 00                      	cmove  0xc8(%rsp),%rax
   85bd4:	ba 43 00 00 00                                  	mov    $0x43,%edx
   85bd9:	49 0f 44 d7                                     	cmove  %r15,%rdx
   85bdd:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
   85be2:	48 89 4e 08                                     	mov    %rcx,0x8(%rsi)
   85be6:	48 89 f1                                        	mov    %rsi,%rcx
   85be9:	48 c7 46 10 00 00 00 00                         	movq   $0x0,0x10(%rsi)
   85bf1:	66 c7 46 1a 00 00                               	movw   $0x0,0x1a(%rsi)
   85bf7:	c7 46 1c 00 00 00 00                            	movl   $0x0,0x1c(%rsi)
   85bfe:	48 89 46 20                                     	mov    %rax,0x20(%rsi)
   85c02:	48 89 56 28                                     	mov    %rdx,0x28(%rsi)
   85c06:	66 c7 46 30 ff ff                               	movw   $0xffff,0x30(%rsi)
   85c0c:	c6 46 34 0b                                     	movb   $0xb,0x34(%rsi)
   85c10:	e9 67 01 00 00                                  	jmp    85d7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   85c15:	b8 32 00 00 00                                  	mov    $0x32,%eax
   85c1a:	48 8d 35 db 80 f9 ff                            	lea    -0x67f25(%rip),%rsi        # 1dcfc <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x45c>
   85c21:	eb 50                                           	jmp    85c73 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x893>
   85c23:	48 83 7c 24 18 bf                               	cmpq   $0xffffffffffffffbf,0x18(%rsp)
   85c29:	77 28                                           	ja     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85c2b:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
   85c30:	48 89 c8                                        	mov    %rcx,%rax
   85c33:	48 83 e0 c0                                     	and    $0xffffffffffffffc0,%rax
   85c37:	48 83 c0 40                                     	add    $0x40,%rax
   85c3b:	48 39 c3                                        	cmp    %rax,%rbx
   85c3e:	48 0f 42 c3                                     	cmovb  %rbx,%rax
   85c42:	48 29 c8                                        	sub    %rcx,%rax
   85c45:	0f 92 c1                                        	setb   %cl
   85c48:	48 3d 00 00 01 00                               	cmp    $0x10000,%rax
   85c4e:	0f 93 c0                                        	setae  %al
   85c51:	08 c8                                           	or     %cl,%al
   85c53:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   85c5d:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   85c62:	e9 11 01 00 00                                  	jmp    85d78 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x998>
   85c67:	b8 35 00 00 00                                  	mov    $0x35,%eax
   85c6c:	48 8d 35 54 80 f9 ff                            	lea    -0x67fac(%rip),%rsi        # 1dcc7 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x427>
   85c73:	48 b9 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rcx
   85c7d:	48 ff c9                                        	dec    %rcx
   85c80:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
   85c85:	48 89 4a 08                                     	mov    %rcx,0x8(%rdx)
   85c89:	48 89 d1                                        	mov    %rdx,%rcx
   85c8c:	48 c7 42 10 00 00 00 00                         	movq   $0x0,0x10(%rdx)
   85c94:	48 89 72 20                                     	mov    %rsi,0x20(%rdx)
   85c98:	48 89 42 28                                     	mov    %rax,0x28(%rdx)
   85c9c:	66 c7 42 30 04 00                               	movw   $0x4,0x30(%rdx)
   85ca2:	c6 42 34 0a                                     	movb   $0xa,0x34(%rdx)
   85ca6:	e9 d1 00 00 00                                  	jmp    85d7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   85cab:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   85cb5:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   85cba:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
   85cbe:	c6 41 17 00                                     	movb   $0x0,0x17(%rcx)
   85cc2:	66 c7 41 15 00 00                               	movw   $0x0,0x15(%rcx)
   85cc8:	c7 41 11 00 00 00 00                            	movl   $0x0,0x11(%rcx)
   85ccf:	e9 a8 00 00 00                                  	jmp    85d7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   85cd4:	48 b9 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rcx
   85cde:	eb a0                                           	jmp    85c80 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x8a0>
   85ce0:	48 83 7c 24 18 bf                               	cmpq   $0xffffffffffffffbf,0x18(%rsp)
   85ce6:	0f 87 67 ff ff ff                               	ja     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85cec:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
   85cf1:	48 89 c8                                        	mov    %rcx,%rax
   85cf4:	48 83 e0 c0                                     	and    $0xffffffffffffffc0,%rax
   85cf8:	48 83 c0 40                                     	add    $0x40,%rax
   85cfc:	48 39 c3                                        	cmp    %rax,%rbx
   85cff:	48 0f 42 c3                                     	cmovb  %rbx,%rax
   85d03:	48 29 c8                                        	sub    %rcx,%rax
   85d06:	0f 82 47 ff ff ff                               	jb     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85d0c:	48 3d ff ff 00 00                               	cmp    $0xffff,%rax
   85d12:	0f 87 3b ff ff ff                               	ja     85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85d18:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
   85d1d:	48 0f af c8                                     	imul   %rax,%rcx
   85d21:	48 81 c1 ff ef ff ff                            	add    $0xffffffffffffefff,%rcx
   85d28:	48 81 f9 00 f0 ff ff                            	cmp    $0xfffffffffffff000,%rcx
   85d2f:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   85d34:	73 38                                           	jae    85d6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   85d36:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   85d40:	48 ff c8                                        	dec    %rax
   85d43:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
   85d47:	48 c7 41 10 00 00 00 00                         	movq   $0x0,0x10(%rcx)
   85d4f:	48 8d 05 08 d8 f8 ff                            	lea    -0x727f8(%rip),%rax        # 1355e <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xc1e>
   85d56:	48 89 41 20                                     	mov    %rax,0x20(%rcx)
   85d5a:	48 c7 41 28 43 00 00 00                         	movq   $0x43,0x28(%rcx)
   85d62:	66 c7 41 30 ff ff                               	movw   $0xffff,0x30(%rcx)
   85d68:	c6 41 34 0b                                     	movb   $0xb,0x34(%rcx)
   85d6c:	eb 0e                                           	jmp    85d7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   85d6e:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   85d78:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
   85d7c:	48 c7 01 01 00 00 00                            	movq   $0x1,(%rcx)
   85d83:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
   85d89:	74 0b                                           	je     85d96 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9b6>
   85d8b:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
   85d90:	ff 15 3a 80 1e 00                               	call   *0x1e803a(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   85d96:	48 83 7c 24 70 00                               	cmpq   $0x0,0x70(%rsp)
   85d9c:	0f 84 10 f7 ff ff                               	je     854b2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xd2>
   85da2:	48 8b 7c 24 78                                  	mov    0x78(%rsp),%rdi
   85da7:	ff 15 23 80 1e 00                               	call   *0x1e8023(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   85dad:	e9 00 f7 ff ff                                  	jmp    854b2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xd2>
   85db2:	48 83 7c 24 18 bf                               	cmpq   $0xffffffffffffffbf,0x18(%rsp)
   85db8:	e9 96 fe ff ff                                  	jmp    85c53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   85dbd:	bf 08 00 00 00                                  	mov    $0x8,%edi
   85dc2:	48 8b 74 24 48                                  	mov    0x48(%rsp),%rsi
   85dc7:	ff 15 cb 7f 1e 00                               	call   *0x1e7fcb(%rip)        # 26dd98 <_DYNAMIC+0x218>
   85dcd:	48 89 c3                                        	mov    %rax,%rbx
   85dd0:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
   85dd6:	75 10                                           	jne    85de8 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xa08>
   85dd8:	48 83 7c 24 70 00                               	cmpq   $0x0,0x70(%rsp)
   85dde:	75 1b                                           	jne    85dfb <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xa1b>
   85de0:	48 89 df                                        	mov    %rbx,%rdi
   85de3:	e8 c8 f6 1d 00                                  	call   2654b0 <_Unwind_Resume@plt>
   85de8:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
   85ded:	ff 15 dd 7f 1e 00                               	call   *0x1e7fdd(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   85df3:	48 83 7c 24 70 00                               	cmpq   $0x0,0x70(%rsp)
   85df9:	74 e5                                           	je     85de0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xa00>
   85dfb:	48 8b 7c 24 78                                  	mov    0x78(%rsp),%rdi
   85e00:	ff 15 ca 7f 1e 00                               	call   *0x1e7fca(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   85e06:	48 89 df                                        	mov    %rbx,%rdi
   85e09:	e8 a2 f6 1d 00                                  	call   2654b0 <_Unwind_Resume@plt>
