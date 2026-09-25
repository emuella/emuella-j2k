Disassembly of section .text:

00000000000e5250 <emuella_j2k_codestream::write_component_packet_header>:
   e5250:	55                                              	push   %rbp
   e5251:	41 57                                           	push   %r15
   e5253:	41 56                                           	push   %r14
   e5255:	41 55                                           	push   %r13
   e5257:	41 54                                           	push   %r12
   e5259:	53                                              	push   %rbx
   e525a:	48 81 ec b8 01 00 00                            	sub    $0x1b8,%rsp
   e5261:	4c 89 8c 24 b8 00 00 00                         	mov    %r9,0xb8(%rsp)
   e5269:	89 cd                                           	mov    %ecx,%ebp
   e526b:	41 89 d4                                        	mov    %edx,%r12d
   e526e:	48 89 fb                                        	mov    %rdi,%rbx
   e5271:	41 0f b7 c4                                     	movzwl %r12w,%eax
   e5275:	44 0f b7 ed                                     	movzwl %bp,%r13d
   e5279:	4c 0f af e8                                     	imul   %rax,%r13
   e527d:	4d 85 ed                                        	test   %r13,%r13
   e5280:	0f 84 09 06 00 00                               	je     e588f <emuella_j2k_codestream::write_component_packet_header+0x63f>
   e5286:	4d 89 c6                                        	mov    %r8,%r14
   e5289:	49 89 f7                                        	mov    %rsi,%r15
   e528c:	4a 8d 04 ad 00 00 00 00                         	lea    0x0(,%r13,4),%rax
   e5294:	48 8d 3c 40                                     	lea    (%rax,%rax,2),%rdi
   e5298:	48 89 7c 24 20                                  	mov    %rdi,0x20(%rsp)
   e529d:	ff 15 15 fb 18 00                               	call   *0x18fb15(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   e52a3:	48 85 c0                                        	test   %rax,%rax
   e52a6:	0f 84 c1 10 00 00                               	je     e636d <emuella_j2k_codestream::write_component_packet_header+0x111d>
   e52ac:	44 89 a4 24 e8 00 00 00                         	mov    %r12d,0xe8(%rsp)
   e52b4:	89 ac 24 ec 00 00 00                            	mov    %ebp,0xec(%rsp)
   e52bb:	4c 89 bc 24 b0 00 00 00                         	mov    %r15,0xb0(%rsp)
   e52c3:	4c 89 b4 24 a8 00 00 00                         	mov    %r14,0xa8(%rsp)
   e52cb:	48 89 5c 24 78                                  	mov    %rbx,0x78(%rsp)
   e52d0:	48 8b 94 24 b8 00 00 00                         	mov    0xb8(%rsp),%rdx
   e52d8:	48 89 d1                                        	mov    %rdx,%rcx
   e52db:	48 c1 e1 05                                     	shl    $0x5,%rcx
   e52df:	48 89 8c 24 c8 00 00 00                         	mov    %rcx,0xc8(%rsp)
   e52e7:	4c 89 ac 24 f0 00 00 00                         	mov    %r13,0xf0(%rsp)
   e52ef:	4c 89 ac 24 90 00 00 00                         	mov    %r13,0x90(%rsp)
   e52f7:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   e52ff:	48 c7 84 24 a0 00 00 00 00 00 00 00             	movq   $0x0,0xa0(%rsp)
   e530b:	48 85 d2                                        	test   %rdx,%rdx
   e530e:	0f 84 87 00 00 00                               	je     e539b <emuella_j2k_codestream::write_component_packet_header+0x14b>
   e5314:	41 bf 08 00 00 00                               	mov    $0x8,%r15d
   e531a:	45 31 f6                                        	xor    %r14d,%r14d
   e531d:	48 8d 9c 24 90 00 00 00                         	lea    0x90(%rsp),%rbx
   e5325:	4c 8b 25 2c 01 19 00                            	mov    0x19012c(%rip),%r12        # 275458 <_DYNAMIC+0x8a8>
   e532c:	45 31 ed                                        	xor    %r13d,%r13d
   e532f:	eb 42                                           	jmp    e5373 <emuella_j2k_codestream::write_component_packet_header+0x123>
   e5331:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e5340:	83 f5 01                                        	xor    $0x1,%ebp
   e5343:	42 c7 44 38 f8 00 00 00 00                      	movl   $0x0,-0x8(%rax,%r15,1)
   e534c:	42 89 6c 38 fc                                  	mov    %ebp,-0x4(%rax,%r15,1)
   e5351:	42 c6 04 38 00                                  	movb   $0x0,(%rax,%r15,1)
   e5356:	49 ff c6                                        	inc    %r14
   e5359:	4c 89 b4 24 a0 00 00 00                         	mov    %r14,0xa0(%rsp)
   e5361:	49 83 c7 0c                                     	add    $0xc,%r15
   e5365:	49 83 c5 20                                     	add    $0x20,%r13
   e5369:	4c 39 ac 24 c8 00 00 00                         	cmp    %r13,0xc8(%rsp)
   e5371:	74 2b                                           	je     e539e <emuella_j2k_codestream::write_component_packet_header+0x14e>
   e5373:	48 8b 8c 24 a8 00 00 00                         	mov    0xa8(%rsp),%rcx
   e537b:	42 0f b6 6c 29 1b                               	movzbl 0x1b(%rcx,%r13,1),%ebp
   e5381:	4c 3b b4 24 90 00 00 00                         	cmp    0x90(%rsp),%r14
   e5389:	75 b5                                           	jne    e5340 <emuella_j2k_codestream::write_component_packet_header+0xf0>
   e538b:	48 89 df                                        	mov    %rbx,%rdi
   e538e:	41 ff d4                                        	call   *%r12
   e5391:	48 8b 84 24 98 00 00 00                         	mov    0x98(%rsp),%rax
   e5399:	eb a5                                           	jmp    e5340 <emuella_j2k_codestream::write_component_packet_header+0xf0>
   e539b:	45 31 f6                                        	xor    %r14d,%r14d
   e539e:	4c 8b a4 24 f0 00 00 00                         	mov    0xf0(%rsp),%r12
   e53a6:	4d 39 e6                                        	cmp    %r12,%r14
   e53a9:	0f 85 c2 04 00 00                               	jne    e5871 <emuella_j2k_codestream::write_component_packet_header+0x621>
   e53af:	48 c7 44 24 60 00 00 00 00                      	movq   $0x0,0x60(%rsp)
   e53b8:	48 c7 44 24 68 08 00 00 00                      	movq   $0x8,0x68(%rsp)
   e53c1:	48 c7 44 24 70 00 00 00 00                      	movq   $0x0,0x70(%rsp)
   e53ca:	4c 8b bc 24 90 00 00 00                         	mov    0x90(%rsp),%r15
   e53d2:	4c 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%r14
   e53da:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e53df:	ff 15 ab fe 18 00                               	call   *0x18feab(%rip)        # 275290 <_DYNAMIC+0x6e0>
   e53e5:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
   e53ed:	48 8b 8c 24 c8 00 00 00                         	mov    0xc8(%rsp),%rcx
   e53f5:	48 01 c8                                        	add    %rcx,%rax
   e53f8:	48 89 84 24 58 01 00 00                         	mov    %rax,0x158(%rsp)
   e5400:	48 8b 5c 24 68                                  	mov    0x68(%rsp),%rbx
   e5405:	4c 89 3b                                        	mov    %r15,(%rbx)
   e5408:	4c 89 73 08                                     	mov    %r14,0x8(%rbx)
   e540c:	4c 89 63 10                                     	mov    %r12,0x10(%rbx)
   e5410:	8b 84 24 e8 00 00 00                            	mov    0xe8(%rsp),%eax
   e5417:	66 89 43 18                                     	mov    %ax,0x18(%rbx)
   e541b:	8b 84 24 ec 00 00 00                            	mov    0xec(%rsp),%eax
   e5422:	66 89 43 1a                                     	mov    %ax,0x1a(%rbx)
   e5426:	48 c7 44 24 70 01 00 00 00                      	movq   $0x1,0x70(%rsp)
   e542f:	41 be 01 00 00 00                               	mov    $0x1,%r14d
   e5435:	4c 8b 25 7c f9 18 00                            	mov    0x18f97c(%rip),%r12        # 274db8 <malloc@GLIBC_2.2.5>
   e543c:	eb 48                                           	jmp    e5486 <emuella_j2k_codestream::write_component_packet_header+0x236>
   e543e:	66 90                                           	xchg   %ax,%ax
   e5440:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e5445:	ff 15 45 fe 18 00                               	call   *0x18fe45(%rip)        # 275290 <_DYNAMIC+0x6e0>
   e544b:	48 8b 5c 24 68                                  	mov    0x68(%rsp),%rbx
   e5450:	4c 89 3c 2b                                     	mov    %r15,(%rbx,%rbp,1)
   e5454:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   e5459:	48 89 44 2b 08                                  	mov    %rax,0x8(%rbx,%rbp,1)
   e545e:	4c 89 6c 2b 10                                  	mov    %r13,0x10(%rbx,%rbp,1)
   e5463:	8b 84 24 c0 00 00 00                            	mov    0xc0(%rsp),%eax
   e546a:	66 89 44 2b 18                                  	mov    %ax,0x18(%rbx,%rbp,1)
   e546f:	8b 44 24 2c                                     	mov    0x2c(%rsp),%eax
   e5473:	66 89 44 2b 1a                                  	mov    %ax,0x1a(%rbx,%rbp,1)
   e5478:	49 ff c6                                        	inc    %r14
   e547b:	4c 89 74 24 70                                  	mov    %r14,0x70(%rsp)
   e5480:	0f 84 0a 05 00 00                               	je     e5990 <emuella_j2k_codestream::write_component_packet_header+0x740>
   e5486:	4c 89 f5                                        	mov    %r14,%rbp
   e5489:	48 c1 e5 05                                     	shl    $0x5,%rbp
   e548d:	0f b7 54 2b f8                                  	movzwl -0x8(%rbx,%rbp,1),%edx
   e5492:	89 d0                                           	mov    %edx,%eax
   e5494:	83 f0 01                                        	xor    $0x1,%eax
   e5497:	0f b7 74 2b fa                                  	movzwl -0x6(%rbx,%rbp,1),%esi
   e549c:	89 f1                                           	mov    %esi,%ecx
   e549e:	83 f1 01                                        	xor    $0x1,%ecx
   e54a1:	66 09 c1                                        	or     %ax,%cx
   e54a4:	0f 84 16 04 00 00                               	je     e58c0 <emuella_j2k_codestream::write_component_packet_header+0x670>
   e54aa:	89 d0                                           	mov    %edx,%eax
   e54ac:	d1 e8                                           	shr    $1,%eax
   e54ae:	48 89 54 24 18                                  	mov    %rdx,0x18(%rsp)
   e54b3:	89 d1                                           	mov    %edx,%ecx
   e54b5:	29 c1                                           	sub    %eax,%ecx
   e54b7:	0f b7 c6                                        	movzwl %si,%eax
   e54ba:	d1 e8                                           	shr    $1,%eax
   e54bc:	89 b4 24 f8 00 00 00                            	mov    %esi,0xf8(%rsp)
   e54c3:	89 f2                                           	mov    %esi,%edx
   e54c5:	29 c2                                           	sub    %eax,%edx
   e54c7:	89 8c 24 c0 00 00 00                            	mov    %ecx,0xc0(%rsp)
   e54ce:	0f b7 c1                                        	movzwl %cx,%eax
   e54d1:	89 54 24 2c                                     	mov    %edx,0x2c(%rsp)
   e54d5:	44 0f b7 fa                                     	movzwl %dx,%r15d
   e54d9:	4c 0f af f8                                     	imul   %rax,%r15
   e54dd:	4d 85 ff                                        	test   %r15,%r15
   e54e0:	74 2e                                           	je     e5510 <emuella_j2k_codestream::write_component_packet_header+0x2c0>
   e54e2:	4a 8d 04 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rax
   e54ea:	4c 8d 2c 40                                     	lea    (%rax,%rax,2),%r13
   e54ee:	4c 89 ef                                        	mov    %r13,%rdi
   e54f1:	41 ff d4                                        	call   *%r12
   e54f4:	48 85 c0                                        	test   %rax,%rax
   e54f7:	0f 84 54 0e 00 00                               	je     e6351 <emuella_j2k_codestream::write_component_packet_header+0x1101>
   e54fd:	49 89 c2                                        	mov    %rax,%r10
   e5500:	eb 14                                           	jmp    e5516 <emuella_j2k_codestream::write_component_packet_header+0x2c6>
   e5502:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e5510:	41 ba 04 00 00 00                               	mov    $0x4,%r10d
   e5516:	4c 89 7c 24 30                                  	mov    %r15,0x30(%rsp)
   e551b:	4c 89 54 24 38                                  	mov    %r10,0x38(%rsp)
   e5520:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   e5529:	66 83 bc 24 f8 00 00 00 00                      	cmpw   $0x0,0xf8(%rsp)
   e5532:	0f 84 e8 02 00 00                               	je     e5820 <emuella_j2k_codestream::write_component_packet_header+0x5d0>
   e5538:	48 8b 54 24 18                                  	mov    0x18(%rsp),%rdx
   e553d:	48 85 d2                                        	test   %rdx,%rdx
   e5540:	0f 84 da 02 00 00                               	je     e5820 <emuella_j2k_codestream::write_component_packet_header+0x5d0>
   e5546:	48 89 ac 24 10 01 00 00                         	mov    %rbp,0x110(%rsp)
   e554e:	48 01 eb                                        	add    %rbp,%rbx
   e5551:	8b 84 24 c0 00 00 00                            	mov    0xc0(%rsp),%eax
   e5558:	66 83 f8 02                                     	cmp    $0x2,%ax
   e555c:	89 c1                                           	mov    %eax,%ecx
   e555e:	b8 01 00 00 00                                  	mov    $0x1,%eax
   e5563:	0f 42 c8                                        	cmovb  %eax,%ecx
   e5566:	89 8c 24 d8 00 00 00                            	mov    %ecx,0xd8(%rsp)
   e556d:	8b 4c 24 2c                                     	mov    0x2c(%rsp),%ecx
   e5571:	66 83 f9 02                                     	cmp    $0x2,%cx
   e5575:	0f 42 c8                                        	cmovb  %eax,%ecx
   e5578:	89 8c 24 8c 00 00 00                            	mov    %ecx,0x8c(%rsp)
   e557f:	8d 04 12                                        	lea    (%rdx,%rdx,1),%eax
   e5582:	48 89 84 24 20 01 00 00                         	mov    %rax,0x120(%rsp)
   e558a:	8d 04 d5 00 00 00 00                            	lea    0x0(,%rdx,8),%eax
   e5591:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e5595:	48 89 84 24 18 01 00 00                         	mov    %rax,0x118(%rsp)
   e559d:	8d 04 95 00 00 00 00                            	lea    0x0(,%rdx,4),%eax
   e55a4:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e55a8:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
   e55b0:	66 b8 02 00                                     	mov    $0x2,%ax
   e55b4:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
   e55ba:	45 31 c9                                        	xor    %r9d,%r9d
   e55bd:	31 ff                                           	xor    %edi,%edi
   e55bf:	48 c7 84 24 d0 00 00 00 00 00 00 00             	movq   $0x0,0xd0(%rsp)
   e55cb:	31 d2                                           	xor    %edx,%edx
   e55cd:	eb 4f                                           	jmp    e561e <emuella_j2k_codestream::write_component_packet_header+0x3ce>
   e55cf:	90                                              	nop
   e55d0:	48 8b 94 24 28 01 00 00                         	mov    0x128(%rsp),%rdx
   e55d8:	ff c2                                           	inc    %edx
   e55da:	48 83 84 24 d0 00 00 00 02                      	addq   $0x2,0xd0(%rsp)
   e55e3:	8b 84 24 00 01 00 00                            	mov    0x100(%rsp),%eax
   e55ea:	83 c0 02                                        	add    $0x2,%eax
   e55ed:	4c 8b 8c 24 30 01 00 00                         	mov    0x130(%rsp),%r9
   e55f5:	4c 03 8c 24 20 01 00 00                         	add    0x120(%rsp),%r9
   e55fd:	4c 8b 84 24 38 01 00 00                         	mov    0x138(%rsp),%r8
   e5605:	4c 03 84 24 18 01 00 00                         	add    0x118(%rsp),%r8
   e560d:	66 3b 94 24 8c 00 00 00                         	cmp    0x8c(%rsp),%dx
   e5615:	48 89 cf                                        	mov    %rcx,%rdi
   e5618:	0f 84 22 02 00 00                               	je     e5840 <emuella_j2k_codestream::write_component_packet_header+0x5f0>
   e561e:	8b b4 24 f8 00 00 00                            	mov    0xf8(%rsp),%esi
   e5625:	66 39 c6                                        	cmp    %ax,%si
   e5628:	89 84 24 00 01 00 00                            	mov    %eax,0x100(%rsp)
   e562f:	0f 42 c6                                        	cmovb  %esi,%eax
   e5632:	8d 0c 12                                        	lea    (%rdx,%rdx,1),%ecx
   e5635:	48 89 94 24 28 01 00 00                         	mov    %rdx,0x128(%rsp)
   e563d:	8d 14 55 02 00 00 00                            	lea    0x2(,%rdx,2),%edx
   e5644:	66 39 d6                                        	cmp    %dx,%si
   e5647:	0f 42 d6                                        	cmovb  %esi,%edx
   e564a:	66 39 d1                                        	cmp    %dx,%cx
   e564d:	4c 89 84 24 38 01 00 00                         	mov    %r8,0x138(%rsp)
   e5655:	4c 89 8c 24 30 01 00 00                         	mov    %r9,0x130(%rsp)
   e565d:	0f 83 4d 01 00 00                               	jae    e57b0 <emuella_j2k_codestream::write_component_packet_header+0x560>
   e5663:	44 0f b7 e8                                     	movzwl %ax,%r13d
   e5667:	66 b8 02 00                                     	mov    $0x2,%ax
   e566b:	45 31 ff                                        	xor    %r15d,%r15d
   e566e:	4c 89 84 24 08 01 00 00                         	mov    %r8,0x108(%rsp)
   e5676:	4c 89 8c 24 e0 00 00 00                         	mov    %r9,0xe0(%rsp)
   e567e:	48 89 f9                                        	mov    %rdi,%rcx
   e5681:	45 31 e4                                        	xor    %r12d,%r12d
   e5684:	eb 62                                           	jmp    e56e8 <emuella_j2k_codestream::write_component_packet_header+0x498>
   e5686:	66 2e 0f 1f 84 00 00 00 00 00                   	cs nopw 0x0(%rax,%rax,1)
   e5690:	41 ff c4                                        	inc    %r12d
   e5693:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
   e5698:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   e569d:	48 8d 04 49                                     	lea    (%rcx,%rcx,2),%rax
   e56a1:	c7 04 82 00 00 00 00                            	movl   $0x0,(%rdx,%rax,4)
   e56a8:	89 6c 82 04                                     	mov    %ebp,0x4(%rdx,%rax,4)
   e56ac:	49 89 d2                                        	mov    %rdx,%r10
   e56af:	c6 44 82 08 00                                  	movb   $0x0,0x8(%rdx,%rax,4)
   e56b4:	48 ff c1                                        	inc    %rcx
   e56b7:	48 89 4c 24 40                                  	mov    %rcx,0x40(%rsp)
   e56bc:	49 83 c7 02                                     	add    $0x2,%r15
   e56c0:	8b 44 24 28                                     	mov    0x28(%rsp),%eax
   e56c4:	83 c0 02                                        	add    $0x2,%eax
   e56c7:	48 83 84 24 e0 00 00 00 02                      	addq   $0x2,0xe0(%rsp)
   e56d0:	48 83 84 24 08 01 00 00 18                      	addq   $0x18,0x108(%rsp)
   e56d9:	66 44 3b a4 24 d8 00 00 00                      	cmp    0xd8(%rsp),%r12w
   e56e2:	0f 84 e8 fe ff ff                               	je     e55d0 <emuella_j2k_codestream::write_component_packet_header+0x380>
   e56e8:	48 89 4c 24 10                                  	mov    %rcx,0x10(%rsp)
   e56ed:	89 44 24 28                                     	mov    %eax,0x28(%rsp)
   e56f1:	0f b7 c0                                        	movzwl %ax,%eax
   e56f4:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
   e56f9:	48 39 c6                                        	cmp    %rax,%rsi
   e56fc:	48 0f 42 c6                                     	cmovb  %rsi,%rax
   e5700:	43 8d 0c 24                                     	lea    (%r12,%r12,1),%ecx
   e5704:	42 8d 14 65 02 00 00 00                         	lea    0x2(,%r12,2),%edx
   e570c:	66 39 d6                                        	cmp    %dx,%si
   e570f:	0f 43 f2                                        	cmovae %edx,%esi
   e5712:	bd ff ff ff ff                                  	mov    $0xffffffff,%ebp
   e5717:	66 39 f1                                        	cmp    %si,%cx
   e571a:	73 6d                                           	jae    e5789 <emuella_j2k_codestream::write_component_packet_header+0x539>
   e571c:	48 8b 73 f0                                     	mov    -0x10(%rbx),%rsi
   e5720:	48 8b 8c 24 08 01 00 00                         	mov    0x108(%rsp),%rcx
   e5728:	48 8b 94 24 e0 00 00 00                         	mov    0xe0(%rsp),%rdx
   e5730:	4c 8b 84 24 d0 00 00 00                         	mov    0xd0(%rsp),%r8
   e5738:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
   e5740:	49 ff c0                                        	inc    %r8
   e5743:	49 89 c9                                        	mov    %rcx,%r9
   e5746:	48 89 d7                                        	mov    %rdx,%rdi
   e5749:	49 89 c2                                        	mov    %rax,%r10
   e574c:	0f 1f 40 00                                     	nopl   0x0(%rax)
   e5750:	48 39 f7                                        	cmp    %rsi,%rdi
   e5753:	0f 83 55 01 00 00                               	jae    e58ae <emuella_j2k_codestream::write_component_packet_header+0x65e>
   e5759:	4c 8b 5b e8                                     	mov    -0x18(%rbx),%r11
   e575d:	47 8b 1c 0b                                     	mov    (%r11,%r9,1),%r11d
   e5761:	41 39 eb                                        	cmp    %ebp,%r11d
   e5764:	41 0f 42 eb                                     	cmovb  %r11d,%ebp
   e5768:	49 ff ca                                        	dec    %r10
   e576b:	48 ff c7                                        	inc    %rdi
   e576e:	49 83 c1 0c                                     	add    $0xc,%r9
   e5772:	4d 39 d7                                        	cmp    %r10,%r15
   e5775:	75 d9                                           	jne    e5750 <emuella_j2k_codestream::write_component_packet_header+0x500>
   e5777:	48 03 54 24 18                                  	add    0x18(%rsp),%rdx
   e577c:	48 03 8c 24 80 00 00 00                         	add    0x80(%rsp),%rcx
   e5784:	4d 39 e8                                        	cmp    %r13,%r8
   e5787:	75 b7                                           	jne    e5740 <emuella_j2k_codestream::write_component_packet_header+0x4f0>
   e5789:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
   e578e:	48 3b 44 24 30                                  	cmp    0x30(%rsp),%rax
   e5793:	0f 85 f7 fe ff ff                               	jne    e5690 <emuella_j2k_codestream::write_component_packet_header+0x440>
   e5799:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e579e:	ff 15 b4 fc 18 00                               	call   *0x18fcb4(%rip)        # 275458 <_DYNAMIC+0x8a8>
   e57a4:	e9 e7 fe ff ff                                  	jmp    e5690 <emuella_j2k_codestream::write_component_packet_header+0x440>
   e57a9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
   e57b0:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
   e57b4:	4c 8d 3c 85 08 00 00 00                         	lea    0x8(,%rax,4),%r15
   e57bc:	8b 84 24 d8 00 00 00                            	mov    0xd8(%rsp),%eax
   e57c3:	89 c5                                           	mov    %eax,%ebp
   e57c5:	48 89 f9                                        	mov    %rdi,%rcx
   e57c8:	eb 35                                           	jmp    e57ff <emuella_j2k_codestream::write_component_packet_header+0x5af>
   e57ca:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
   e57d0:	48 b8 00 00 00 00 ff ff ff ff                   	movabs $0xffffffff00000000,%rax
   e57da:	4a 89 44 39 f8                                  	mov    %rax,-0x8(%rcx,%r15,1)
   e57df:	49 89 ca                                        	mov    %rcx,%r10
   e57e2:	42 c6 04 39 00                                  	movb   $0x0,(%rcx,%r15,1)
   e57e7:	49 ff c4                                        	inc    %r12
   e57ea:	4c 89 64 24 40                                  	mov    %r12,0x40(%rsp)
   e57ef:	49 83 c7 0c                                     	add    $0xc,%r15
   e57f3:	66 ff cd                                        	dec    %bp
   e57f6:	4c 89 e1                                        	mov    %r12,%rcx
   e57f9:	0f 84 d1 fd ff ff                               	je     e55d0 <emuella_j2k_codestream::write_component_packet_header+0x380>
   e57ff:	48 3b 4c 24 30                                  	cmp    0x30(%rsp),%rcx
   e5804:	49 89 cc                                        	mov    %rcx,%r12
   e5807:	4c 89 d1                                        	mov    %r10,%rcx
   e580a:	75 c4                                           	jne    e57d0 <emuella_j2k_codestream::write_component_packet_header+0x580>
   e580c:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5811:	ff 15 41 fc 18 00                               	call   *0x18fc41(%rip)        # 275458 <_DYNAMIC+0x8a8>
   e5817:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
   e581c:	eb b2                                           	jmp    e57d0 <emuella_j2k_codestream::write_component_packet_header+0x580>
   e581e:	66 90                                           	xchg   %ax,%ax
   e5820:	4c 89 54 24 18                                  	mov    %r10,0x18(%rsp)
   e5825:	45 31 ed                                        	xor    %r13d,%r13d
   e5828:	4c 3b 74 24 60                                  	cmp    0x60(%rsp),%r14
   e582d:	0f 84 0d fc ff ff                               	je     e5440 <emuella_j2k_codestream::write_component_packet_header+0x1f0>
   e5833:	e9 13 fc ff ff                                  	jmp    e544b <emuella_j2k_codestream::write_component_packet_header+0x1fb>
   e5838:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
   e5840:	49 89 fd                                        	mov    %rdi,%r13
   e5843:	4c 8b 7c 24 30                                  	mov    0x30(%rsp),%r15
   e5848:	48 8b 44 24 38                                  	mov    0x38(%rsp),%rax
   e584d:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   e5852:	4c 8b 25 5f f5 18 00                            	mov    0x18f55f(%rip),%r12        # 274db8 <malloc@GLIBC_2.2.5>
   e5859:	48 8b ac 24 10 01 00 00                         	mov    0x110(%rsp),%rbp
   e5861:	4c 3b 74 24 60                                  	cmp    0x60(%rsp),%r14
   e5866:	0f 85 df fb ff ff                               	jne    e544b <emuella_j2k_codestream::write_component_packet_header+0x1fb>
   e586c:	e9 cf fb ff ff                                  	jmp    e5440 <emuella_j2k_codestream::write_component_packet_header+0x1f0>
   e5871:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e587a:	74 0e                                           	je     e588a <emuella_j2k_codestream::write_component_packet_header+0x63a>
   e587c:	48 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%rdi
   e5884:	ff 15 3e f5 18 00                               	call   *0x18f53e(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e588a:	48 8b 5c 24 78                                  	mov    0x78(%rsp),%rbx
   e588f:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e5899:	48 89 03                                        	mov    %rax,(%rbx)
   e589c:	48 81 c4 b8 01 00 00                            	add    $0x1b8,%rsp
   e58a3:	5b                                              	pop    %rbx
   e58a4:	41 5c                                           	pop    %r12
   e58a6:	41 5d                                           	pop    %r13
   e58a8:	41 5e                                           	pop    %r14
   e58aa:	41 5f                                           	pop    %r15
   e58ac:	5d                                              	pop    %rbp
   e58ad:	c3                                              	ret
   e58ae:	48 8d 15 43 86 18 00                            	lea    0x188643(%rip),%rdx        # 26def8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x280>
   e58b5:	ff 15 55 f5 18 00                               	call   *0x18f555(%rip)        # 274e10 <_DYNAMIC+0x260>
   e58bb:	e9 db 0a 00 00                                  	jmp    e639b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e58c0:	48 8b 6c 24 60                                  	mov    0x60(%rsp),%rbp
   e58c5:	4c 8b 7c 24 20                                  	mov    0x20(%rsp),%r15
   e58ca:	4c 89 ff                                        	mov    %r15,%rdi
   e58cd:	ff 15 e5 f4 18 00                               	call   *0x18f4e5(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   e58d3:	48 85 c0                                        	test   %rax,%rax
   e58d6:	0f 84 b1 0a 00 00                               	je     e638d <emuella_j2k_codestream::write_component_packet_header+0x113d>
   e58dc:	48 89 6c 24 20                                  	mov    %rbp,0x20(%rsp)
   e58e1:	48 8b 8c 24 f0 00 00 00                         	mov    0xf0(%rsp),%rcx
   e58e9:	48 89 8c 24 90 00 00 00                         	mov    %rcx,0x90(%rsp)
   e58f1:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   e58f9:	48 c7 84 24 a0 00 00 00 00 00 00 00             	movq   $0x0,0xa0(%rsp)
   e5905:	48 83 bc 24 b8 00 00 00 00                      	cmpq   $0x0,0xb8(%rsp)
   e590e:	0f 84 90 00 00 00                               	je     e59a4 <emuella_j2k_codestream::write_component_packet_header+0x754>
   e5914:	41 bd 08 00 00 00                               	mov    $0x8,%r13d
   e591a:	45 31 e4                                        	xor    %r12d,%r12d
   e591d:	31 ed                                           	xor    %ebp,%ebp
   e591f:	eb 3f                                           	jmp    e5960 <emuella_j2k_codestream::write_component_packet_header+0x710>
   e5921:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e5930:	42 c7 44 28 f8 00 00 00 00                      	movl   $0x0,-0x8(%rax,%r13,1)
   e5939:	46 89 7c 28 fc                                  	mov    %r15d,-0x4(%rax,%r13,1)
   e593e:	42 c6 04 28 00                                  	movb   $0x0,(%rax,%r13,1)
   e5943:	49 ff c4                                        	inc    %r12
   e5946:	4c 89 a4 24 a0 00 00 00                         	mov    %r12,0xa0(%rsp)
   e594e:	49 83 c5 0c                                     	add    $0xc,%r13
   e5952:	48 83 c5 20                                     	add    $0x20,%rbp
   e5956:	48 39 ac 24 c8 00 00 00                         	cmp    %rbp,0xc8(%rsp)
   e595e:	74 47                                           	je     e59a7 <emuella_j2k_codestream::write_component_packet_header+0x757>
   e5960:	48 8b 8c 24 a8 00 00 00                         	mov    0xa8(%rsp),%rcx
   e5968:	44 0f b6 7c 29 1a                               	movzbl 0x1a(%rcx,%rbp,1),%r15d
   e596e:	4c 3b a4 24 90 00 00 00                         	cmp    0x90(%rsp),%r12
   e5976:	75 b8                                           	jne    e5930 <emuella_j2k_codestream::write_component_packet_header+0x6e0>
   e5978:	48 8d bc 24 90 00 00 00                         	lea    0x90(%rsp),%rdi
   e5980:	ff 15 d2 fa 18 00                               	call   *0x18fad2(%rip)        # 275458 <_DYNAMIC+0x8a8>
   e5986:	48 8b 84 24 98 00 00 00                         	mov    0x98(%rsp),%rax
   e598e:	eb a0                                           	jmp    e5930 <emuella_j2k_codestream::write_component_packet_header+0x6e0>
   e5990:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e5996:	0f 84 ee fe ff ff                               	je     e588a <emuella_j2k_codestream::write_component_packet_header+0x63a>
   e599c:	48 89 df                                        	mov    %rbx,%rdi
   e599f:	e9 e0 fe ff ff                                  	jmp    e5884 <emuella_j2k_codestream::write_component_packet_header+0x634>
   e59a4:	45 31 e4                                        	xor    %r12d,%r12d
   e59a7:	4c 3b a4 24 f0 00 00 00                         	cmp    0xf0(%rsp),%r12
   e59af:	0f 85 ba 04 00 00                               	jne    e5e6f <emuella_j2k_codestream::write_component_packet_header+0xc1f>
   e59b5:	48 c7 44 24 60 00 00 00 00                      	movq   $0x0,0x60(%rsp)
   e59be:	48 c7 44 24 68 08 00 00 00                      	movq   $0x8,0x68(%rsp)
   e59c7:	48 c7 44 24 70 00 00 00 00                      	movq   $0x0,0x70(%rsp)
   e59d0:	4c 8b ac 24 90 00 00 00                         	mov    0x90(%rsp),%r13
   e59d8:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
   e59e0:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e59e5:	ff 15 a5 f8 18 00                               	call   *0x18f8a5(%rip)        # 275290 <_DYNAMIC+0x6e0>
   e59eb:	48 8b 6c 24 68                                  	mov    0x68(%rsp),%rbp
   e59f0:	4c 89 6d 00                                     	mov    %r13,0x0(%rbp)
   e59f4:	4c 89 7d 08                                     	mov    %r15,0x8(%rbp)
   e59f8:	48 8b 84 24 f0 00 00 00                         	mov    0xf0(%rsp),%rax
   e5a00:	48 89 45 10                                     	mov    %rax,0x10(%rbp)
   e5a04:	8b 84 24 e8 00 00 00                            	mov    0xe8(%rsp),%eax
   e5a0b:	66 89 45 18                                     	mov    %ax,0x18(%rbp)
   e5a0f:	8b 84 24 ec 00 00 00                            	mov    0xec(%rsp),%eax
   e5a16:	66 89 45 1a                                     	mov    %ax,0x1a(%rbp)
   e5a1a:	48 c7 44 24 70 01 00 00 00                      	movq   $0x1,0x70(%rsp)
   e5a23:	41 bd 01 00 00 00                               	mov    $0x1,%r13d
   e5a29:	eb 4f                                           	jmp    e5a7a <emuella_j2k_codestream::write_component_packet_header+0x82a>
   e5a2b:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e5a30:	ff 15 5a f8 18 00                               	call   *0x18f85a(%rip)        # 275290 <_DYNAMIC+0x6e0>
   e5a36:	48 8b 6c 24 68                                  	mov    0x68(%rsp),%rbp
   e5a3b:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
   e5a43:	4c 89 7c 05 00                                  	mov    %r15,0x0(%rbp,%rax,1)
   e5a48:	4c 89 64 05 08                                  	mov    %r12,0x8(%rbp,%rax,1)
   e5a4d:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   e5a52:	48 89 4c 05 10                                  	mov    %rcx,0x10(%rbp,%rax,1)
   e5a57:	8b 4c 24 2c                                     	mov    0x2c(%rsp),%ecx
   e5a5b:	66 89 4c 05 18                                  	mov    %cx,0x18(%rbp,%rax,1)
   e5a60:	8b 8c 24 b8 00 00 00                            	mov    0xb8(%rsp),%ecx
   e5a67:	66 89 4c 05 1a                                  	mov    %cx,0x1a(%rbp,%rax,1)
   e5a6c:	49 ff c5                                        	inc    %r13
   e5a6f:	4c 89 6c 24 70                                  	mov    %r13,0x70(%rsp)
   e5a74:	0f 84 4a 06 00 00                               	je     e60c4 <emuella_j2k_codestream::write_component_packet_header+0xe74>
   e5a7a:	4c 89 e9                                        	mov    %r13,%rcx
   e5a7d:	48 c1 e1 05                                     	shl    $0x5,%rcx
   e5a81:	0f b7 54 0d f8                                  	movzwl -0x8(%rbp,%rcx,1),%edx
   e5a86:	89 d0                                           	mov    %edx,%eax
   e5a88:	83 f0 01                                        	xor    $0x1,%eax
   e5a8b:	48 89 8c 24 c0 00 00 00                         	mov    %rcx,0xc0(%rsp)
   e5a93:	0f b7 74 0d fa                                  	movzwl -0x6(%rbp,%rcx,1),%esi
   e5a98:	89 f1                                           	mov    %esi,%ecx
   e5a9a:	83 f1 01                                        	xor    $0x1,%ecx
   e5a9d:	66 09 c1                                        	or     %ax,%cx
   e5aa0:	0f 84 02 04 00 00                               	je     e5ea8 <emuella_j2k_codestream::write_component_packet_header+0xc58>
   e5aa6:	89 d0                                           	mov    %edx,%eax
   e5aa8:	d1 e8                                           	shr    $1,%eax
   e5aaa:	48 89 54 24 18                                  	mov    %rdx,0x18(%rsp)
   e5aaf:	89 d1                                           	mov    %edx,%ecx
   e5ab1:	29 c1                                           	sub    %eax,%ecx
   e5ab3:	0f b7 c6                                        	movzwl %si,%eax
   e5ab6:	d1 e8                                           	shr    $1,%eax
   e5ab8:	89 b4 24 8c 00 00 00                            	mov    %esi,0x8c(%rsp)
   e5abf:	89 f2                                           	mov    %esi,%edx
   e5ac1:	29 c2                                           	sub    %eax,%edx
   e5ac3:	89 4c 24 2c                                     	mov    %ecx,0x2c(%rsp)
   e5ac7:	0f b7 c1                                        	movzwl %cx,%eax
   e5aca:	89 94 24 b8 00 00 00                            	mov    %edx,0xb8(%rsp)
   e5ad1:	44 0f b7 fa                                     	movzwl %dx,%r15d
   e5ad5:	4c 0f af f8                                     	imul   %rax,%r15
   e5ad9:	4d 85 ff                                        	test   %r15,%r15
   e5adc:	74 1f                                           	je     e5afd <emuella_j2k_codestream::write_component_packet_header+0x8ad>
   e5ade:	4a 8d 04 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rax
   e5ae6:	4c 8d 24 40                                     	lea    (%rax,%rax,2),%r12
   e5aea:	4c 89 e7                                        	mov    %r12,%rdi
   e5aed:	ff 15 c5 f2 18 00                               	call   *0x18f2c5(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   e5af3:	48 85 c0                                        	test   %rax,%rax
   e5af6:	75 0a                                           	jne    e5b02 <emuella_j2k_codestream::write_component_packet_header+0x8b2>
   e5af8:	e9 80 08 00 00                                  	jmp    e637d <emuella_j2k_codestream::write_component_packet_header+0x112d>
   e5afd:	b8 04 00 00 00                                  	mov    $0x4,%eax
   e5b02:	4c 89 7c 24 30                                  	mov    %r15,0x30(%rsp)
   e5b07:	48 89 44 24 38                                  	mov    %rax,0x38(%rsp)
   e5b0c:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   e5b15:	66 83 bc 24 8c 00 00 00 00                      	cmpw   $0x0,0x8c(%rsp)
   e5b1e:	0f 84 05 03 00 00                               	je     e5e29 <emuella_j2k_codestream::write_component_packet_header+0xbd9>
   e5b24:	48 8b 54 24 18                                  	mov    0x18(%rsp),%rdx
   e5b29:	48 85 d2                                        	test   %rdx,%rdx
   e5b2c:	0f 84 f7 02 00 00                               	je     e5e29 <emuella_j2k_codestream::write_component_packet_header+0xbd9>
   e5b32:	49 89 c2                                        	mov    %rax,%r10
   e5b35:	4c 89 ac 24 10 01 00 00                         	mov    %r13,0x110(%rsp)
   e5b3d:	48 03 ac 24 c0 00 00 00                         	add    0xc0(%rsp),%rbp
   e5b45:	8b 44 24 2c                                     	mov    0x2c(%rsp),%eax
   e5b49:	66 83 f8 02                                     	cmp    $0x2,%ax
   e5b4d:	89 c1                                           	mov    %eax,%ecx
   e5b4f:	b8 01 00 00 00                                  	mov    $0x1,%eax
   e5b54:	0f 42 c8                                        	cmovb  %eax,%ecx
   e5b57:	89 8c 24 d0 00 00 00                            	mov    %ecx,0xd0(%rsp)
   e5b5e:	8b 8c 24 b8 00 00 00                            	mov    0xb8(%rsp),%ecx
   e5b65:	66 83 f9 02                                     	cmp    $0x2,%cx
   e5b69:	0f 42 c8                                        	cmovb  %eax,%ecx
   e5b6c:	89 8c 24 20 01 00 00                            	mov    %ecx,0x120(%rsp)
   e5b73:	8d 04 12                                        	lea    (%rdx,%rdx,1),%eax
   e5b76:	48 89 84 24 18 01 00 00                         	mov    %rax,0x118(%rsp)
   e5b7e:	8d 04 d5 00 00 00 00                            	lea    0x0(,%rdx,8),%eax
   e5b85:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e5b89:	48 89 84 24 c8 00 00 00                         	mov    %rax,0xc8(%rsp)
   e5b91:	8d 04 95 00 00 00 00                            	lea    0x0(,%rdx,4),%eax
   e5b98:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e5b9c:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
   e5ba4:	66 b8 02 00                                     	mov    $0x2,%ax
   e5ba8:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
   e5bae:	45 31 c9                                        	xor    %r9d,%r9d
   e5bb1:	31 ff                                           	xor    %edi,%edi
   e5bb3:	48 c7 84 24 00 01 00 00 00 00 00 00             	movq   $0x0,0x100(%rsp)
   e5bbf:	31 d2                                           	xor    %edx,%edx
   e5bc1:	eb 5b                                           	jmp    e5c1e <emuella_j2k_codestream::write_component_packet_header+0x9ce>
   e5bc3:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e5bd0:	48 8b 94 24 f8 00 00 00                         	mov    0xf8(%rsp),%rdx
   e5bd8:	ff c2                                           	inc    %edx
   e5bda:	48 83 84 24 00 01 00 00 02                      	addq   $0x2,0x100(%rsp)
   e5be3:	8b 84 24 38 01 00 00                            	mov    0x138(%rsp),%eax
   e5bea:	83 c0 02                                        	add    $0x2,%eax
   e5bed:	4c 8b 8c 24 28 01 00 00                         	mov    0x128(%rsp),%r9
   e5bf5:	4c 03 8c 24 18 01 00 00                         	add    0x118(%rsp),%r9
   e5bfd:	4c 8b 84 24 30 01 00 00                         	mov    0x130(%rsp),%r8
   e5c05:	4c 03 84 24 c8 00 00 00                         	add    0xc8(%rsp),%r8
   e5c0d:	66 3b 94 24 20 01 00 00                         	cmp    0x120(%rsp),%dx
   e5c15:	48 89 cf                                        	mov    %rcx,%rdi
   e5c18:	0f 84 27 02 00 00                               	je     e5e45 <emuella_j2k_codestream::write_component_packet_header+0xbf5>
   e5c1e:	8b b4 24 8c 00 00 00                            	mov    0x8c(%rsp),%esi
   e5c25:	66 39 c6                                        	cmp    %ax,%si
   e5c28:	89 84 24 38 01 00 00                            	mov    %eax,0x138(%rsp)
   e5c2f:	0f 42 c6                                        	cmovb  %esi,%eax
   e5c32:	8d 0c 12                                        	lea    (%rdx,%rdx,1),%ecx
   e5c35:	48 89 94 24 f8 00 00 00                         	mov    %rdx,0xf8(%rsp)
   e5c3d:	8d 14 55 02 00 00 00                            	lea    0x2(,%rdx,2),%edx
   e5c44:	66 39 d6                                        	cmp    %dx,%si
   e5c47:	0f 42 d6                                        	cmovb  %esi,%edx
   e5c4a:	66 39 d1                                        	cmp    %dx,%cx
   e5c4d:	4c 89 84 24 30 01 00 00                         	mov    %r8,0x130(%rsp)
   e5c55:	4c 89 8c 24 28 01 00 00                         	mov    %r9,0x128(%rsp)
   e5c5d:	0f 83 5d 01 00 00                               	jae    e5dc0 <emuella_j2k_codestream::write_component_packet_header+0xb70>
   e5c63:	48 89 f9                                        	mov    %rdi,%rcx
   e5c66:	44 0f b7 e8                                     	movzwl %ax,%r13d
   e5c6a:	66 b8 02 00                                     	mov    $0x2,%ax
   e5c6e:	45 31 e4                                        	xor    %r12d,%r12d
   e5c71:	4c 89 84 24 e0 00 00 00                         	mov    %r8,0xe0(%rsp)
   e5c79:	4c 89 8c 24 d8 00 00 00                         	mov    %r9,0xd8(%rsp)
   e5c81:	31 d2                                           	xor    %edx,%edx
   e5c83:	eb 69                                           	jmp    e5cee <emuella_j2k_codestream::write_component_packet_header+0xa9e>
   e5c85:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
   e5c90:	48 8b 94 24 08 01 00 00                         	mov    0x108(%rsp),%rdx
   e5c98:	ff c2                                           	inc    %edx
   e5c9a:	4c 8b 54 24 38                                  	mov    0x38(%rsp),%r10
   e5c9f:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   e5ca4:	48 8d 04 49                                     	lea    (%rcx,%rcx,2),%rax
   e5ca8:	41 c7 04 82 00 00 00 00                         	movl   $0x0,(%r10,%rax,4)
   e5cb0:	45 89 7c 82 04                                  	mov    %r15d,0x4(%r10,%rax,4)
   e5cb5:	41 c6 44 82 08 00                               	movb   $0x0,0x8(%r10,%rax,4)
   e5cbb:	48 ff c1                                        	inc    %rcx
   e5cbe:	48 89 4c 24 40                                  	mov    %rcx,0x40(%rsp)
   e5cc3:	49 83 c4 02                                     	add    $0x2,%r12
   e5cc7:	8b 44 24 28                                     	mov    0x28(%rsp),%eax
   e5ccb:	83 c0 02                                        	add    $0x2,%eax
   e5cce:	48 83 84 24 d8 00 00 00 02                      	addq   $0x2,0xd8(%rsp)
   e5cd7:	48 83 84 24 e0 00 00 00 18                      	addq   $0x18,0xe0(%rsp)
   e5ce0:	66 3b 94 24 d0 00 00 00                         	cmp    0xd0(%rsp),%dx
   e5ce8:	0f 84 e2 fe ff ff                               	je     e5bd0 <emuella_j2k_codestream::write_component_packet_header+0x980>
   e5cee:	48 89 4c 24 10                                  	mov    %rcx,0x10(%rsp)
   e5cf3:	89 44 24 28                                     	mov    %eax,0x28(%rsp)
   e5cf7:	0f b7 c0                                        	movzwl %ax,%eax
   e5cfa:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
   e5cff:	48 39 c6                                        	cmp    %rax,%rsi
   e5d02:	48 0f 42 c6                                     	cmovb  %rsi,%rax
   e5d06:	8d 0c 12                                        	lea    (%rdx,%rdx,1),%ecx
   e5d09:	48 89 94 24 08 01 00 00                         	mov    %rdx,0x108(%rsp)
   e5d11:	8d 14 55 02 00 00 00                            	lea    0x2(,%rdx,2),%edx
   e5d18:	66 39 d6                                        	cmp    %dx,%si
   e5d1b:	0f 43 f2                                        	cmovae %edx,%esi
   e5d1e:	41 bf ff ff ff ff                               	mov    $0xffffffff,%r15d
   e5d24:	66 39 f1                                        	cmp    %si,%cx
   e5d27:	73 70                                           	jae    e5d99 <emuella_j2k_codestream::write_component_packet_header+0xb49>
   e5d29:	48 8b 75 f0                                     	mov    -0x10(%rbp),%rsi
   e5d2d:	48 8b 8c 24 e0 00 00 00                         	mov    0xe0(%rsp),%rcx
   e5d35:	48 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%rdx
   e5d3d:	4c 8b 84 24 00 01 00 00                         	mov    0x100(%rsp),%r8
   e5d45:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
   e5d50:	49 ff c0                                        	inc    %r8
   e5d53:	49 89 c9                                        	mov    %rcx,%r9
   e5d56:	48 89 d7                                        	mov    %rdx,%rdi
   e5d59:	49 89 c2                                        	mov    %rax,%r10
   e5d5c:	0f 1f 40 00                                     	nopl   0x0(%rax)
   e5d60:	48 39 f7                                        	cmp    %rsi,%rdi
   e5d63:	0f 83 2d 01 00 00                               	jae    e5e96 <emuella_j2k_codestream::write_component_packet_header+0xc46>
   e5d69:	4c 8b 5d e8                                     	mov    -0x18(%rbp),%r11
   e5d6d:	47 8b 1c 0b                                     	mov    (%r11,%r9,1),%r11d
   e5d71:	45 39 fb                                        	cmp    %r15d,%r11d
   e5d74:	45 0f 42 fb                                     	cmovb  %r11d,%r15d
   e5d78:	49 ff ca                                        	dec    %r10
   e5d7b:	48 ff c7                                        	inc    %rdi
   e5d7e:	49 83 c1 0c                                     	add    $0xc,%r9
   e5d82:	4d 39 d4                                        	cmp    %r10,%r12
   e5d85:	75 d9                                           	jne    e5d60 <emuella_j2k_codestream::write_component_packet_header+0xb10>
   e5d87:	48 03 54 24 18                                  	add    0x18(%rsp),%rdx
   e5d8c:	48 03 8c 24 80 00 00 00                         	add    0x80(%rsp),%rcx
   e5d94:	4d 39 e8                                        	cmp    %r13,%r8
   e5d97:	75 b7                                           	jne    e5d50 <emuella_j2k_codestream::write_component_packet_header+0xb00>
   e5d99:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
   e5d9e:	48 3b 44 24 30                                  	cmp    0x30(%rsp),%rax
   e5da3:	0f 85 e7 fe ff ff                               	jne    e5c90 <emuella_j2k_codestream::write_component_packet_header+0xa40>
   e5da9:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5dae:	ff 15 a4 f6 18 00                               	call   *0x18f6a4(%rip)        # 275458 <_DYNAMIC+0x8a8>
   e5db4:	e9 d7 fe ff ff                                  	jmp    e5c90 <emuella_j2k_codestream::write_component_packet_header+0xa40>
   e5db9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
   e5dc0:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
   e5dc4:	4c 8d 3c 85 08 00 00 00                         	lea    0x8(,%rax,4),%r15
   e5dcc:	8b 84 24 d0 00 00 00                            	mov    0xd0(%rsp),%eax
   e5dd3:	41 89 c4                                        	mov    %eax,%r12d
   e5dd6:	48 89 f9                                        	mov    %rdi,%rcx
   e5dd9:	eb 32                                           	jmp    e5e0d <emuella_j2k_codestream::write_component_packet_header+0xbbd>
   e5ddb:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
   e5de0:	48 b8 00 00 00 00 ff ff ff ff                   	movabs $0xffffffff00000000,%rax
   e5dea:	4b 89 44 3a f8                                  	mov    %rax,-0x8(%r10,%r15,1)
   e5def:	43 c6 04 3a 00                                  	movb   $0x0,(%r10,%r15,1)
   e5df4:	49 ff c5                                        	inc    %r13
   e5df7:	4c 89 6c 24 40                                  	mov    %r13,0x40(%rsp)
   e5dfc:	49 83 c7 0c                                     	add    $0xc,%r15
   e5e00:	66 41 ff cc                                     	dec    %r12w
   e5e04:	4c 89 e9                                        	mov    %r13,%rcx
   e5e07:	0f 84 c3 fd ff ff                               	je     e5bd0 <emuella_j2k_codestream::write_component_packet_header+0x980>
   e5e0d:	48 3b 4c 24 30                                  	cmp    0x30(%rsp),%rcx
   e5e12:	49 89 cd                                        	mov    %rcx,%r13
   e5e15:	75 c9                                           	jne    e5de0 <emuella_j2k_codestream::write_component_packet_header+0xb90>
   e5e17:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5e1c:	ff 15 36 f6 18 00                               	call   *0x18f636(%rip)        # 275458 <_DYNAMIC+0x8a8>
   e5e22:	4c 8b 54 24 38                                  	mov    0x38(%rsp),%r10
   e5e27:	eb b7                                           	jmp    e5de0 <emuella_j2k_codestream::write_component_packet_header+0xb90>
   e5e29:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
   e5e32:	49 89 c4                                        	mov    %rax,%r12
   e5e35:	4c 3b 6c 24 60                                  	cmp    0x60(%rsp),%r13
   e5e3a:	0f 85 f6 fb ff ff                               	jne    e5a36 <emuella_j2k_codestream::write_component_packet_header+0x7e6>
   e5e40:	e9 e6 fb ff ff                                  	jmp    e5a2b <emuella_j2k_codestream::write_component_packet_header+0x7db>
   e5e45:	48 89 7c 24 10                                  	mov    %rdi,0x10(%rsp)
   e5e4a:	4c 8b 7c 24 30                                  	mov    0x30(%rsp),%r15
   e5e4f:	48 8b 44 24 38                                  	mov    0x38(%rsp),%rax
   e5e54:	4c 8b ac 24 10 01 00 00                         	mov    0x110(%rsp),%r13
   e5e5c:	49 89 c4                                        	mov    %rax,%r12
   e5e5f:	4c 3b 6c 24 60                                  	cmp    0x60(%rsp),%r13
   e5e64:	0f 84 c1 fb ff ff                               	je     e5a2b <emuella_j2k_codestream::write_component_packet_header+0x7db>
   e5e6a:	e9 c7 fb ff ff                                  	jmp    e5a36 <emuella_j2k_codestream::write_component_packet_header+0x7e6>
   e5e6f:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e5e78:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e5e7d:	0f 84 57 02 00 00                               	je     e60da <emuella_j2k_codestream::write_component_packet_header+0xe8a>
   e5e83:	48 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%rdi
   e5e8b:	ff 15 37 ef 18 00                               	call   *0x18ef37(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e5e91:	e9 44 02 00 00                                  	jmp    e60da <emuella_j2k_codestream::write_component_packet_header+0xe8a>
   e5e96:	48 8d 15 5b 80 18 00                            	lea    0x18805b(%rip),%rdx        # 26def8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x280>
   e5e9d:	ff 15 6d ef 18 00                               	call   *0x18ef6d(%rip)        # 274e10 <_DYNAMIC+0x260>
   e5ea3:	e9 f3 04 00 00                                  	jmp    e639b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e5ea8:	48 8b 44 24 70                                  	mov    0x70(%rsp),%rax
   e5ead:	0f 10 44 24 60                                  	movups 0x60(%rsp),%xmm0
   e5eb2:	0f 29 84 24 80 01 00 00                         	movaps %xmm0,0x180(%rsp)
   e5eba:	0f 29 84 24 40 01 00 00                         	movaps %xmm0,0x140(%rsp)
   e5ec2:	48 89 44 24 10                                  	mov    %rax,0x10(%rsp)
   e5ec7:	48 89 84 24 50 01 00 00                         	mov    %rax,0x150(%rsp)
   e5ecf:	48 8b 84 24 48 01 00 00                         	mov    0x148(%rsp),%rax
   e5ed7:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
   e5edf:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
   e5ee7:	48 3b 84 24 58 01 00 00                         	cmp    0x158(%rsp),%rax
   e5eef:	0f 84 7b 02 00 00                               	je     e6170 <emuella_j2k_codestream::write_component_packet_header+0xf20>
   e5ef5:	44 0f b6 60 1b                                  	movzbl 0x1b(%rax),%r12d
   e5efa:	44 0f b7 78 10                                  	movzwl 0x10(%rax),%r15d
   e5eff:	49 89 c5                                        	mov    %rax,%r13
   e5f02:	0f b7 68 12                                     	movzwl 0x12(%rax),%ebp
   e5f06:	c7 04 24 01 00 00 00                            	movl   $0x1,(%rsp)
   e5f0d:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5f12:	48 89 de                                        	mov    %rbx,%rsi
   e5f15:	4c 89 f2                                        	mov    %r14,%rdx
   e5f18:	48 8b 8c 24 b0 00 00 00                         	mov    0xb0(%rsp),%rcx
   e5f20:	45 89 f8                                        	mov    %r15d,%r8d
   e5f23:	41 89 e9                                        	mov    %ebp,%r9d
   e5f26:	e8 45 65 06 00                                  	call   14c470 <<emuella_j2k_codestream::EncTagTree>::encode>
   e5f2b:	48 83 7c 24 30 ff                               	cmpq   $0xffffffffffffffff,0x30(%rsp)
   e5f31:	0f 85 15 02 00 00                               	jne    e614c <emuella_j2k_codestream::write_component_packet_header+0xefc>
   e5f37:	49 8d 45 20                                     	lea    0x20(%r13),%rax
   e5f3b:	45 84 e4                                        	test   %r12b,%r12b
   e5f3e:	74 a7                                           	je     e5ee7 <emuella_j2k_codestream::write_component_packet_header+0xc97>
   e5f40:	c7 04 24 ff ff ff ff                            	movl   $0xffffffff,(%rsp)
   e5f47:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5f4c:	48 8b b4 24 80 00 00 00                         	mov    0x80(%rsp),%rsi
   e5f54:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
   e5f59:	48 8b 8c 24 b0 00 00 00                         	mov    0xb0(%rsp),%rcx
   e5f61:	45 89 f8                                        	mov    %r15d,%r8d
   e5f64:	41 89 e9                                        	mov    %ebp,%r9d
   e5f67:	e8 04 65 06 00                                  	call   14c470 <<emuella_j2k_codestream::EncTagTree>::encode>
   e5f6c:	48 83 7c 24 30 ff                               	cmpq   $0xffffffffffffffff,0x30(%rsp)
   e5f72:	0f 85 d4 01 00 00                               	jne    e614c <emuella_j2k_codestream::write_component_packet_header+0xefc>
   e5f78:	49 83 c5 20                                     	add    $0x20,%r13
   e5f7c:	4c 89 ac 24 a8 00 00 00                         	mov    %r13,0xa8(%rsp)
   e5f84:	41 0f b7 6d f8                                  	movzwl -0x8(%r13),%ebp
   e5f89:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5f8e:	48 8b b4 24 b0 00 00 00                         	mov    0xb0(%rsp),%rsi
   e5f96:	89 ea                                           	mov    %ebp,%edx
   e5f98:	e8 63 0a ff ff                                  	call   d6a00 <emuella_j2k_codestream::write_coding_pass_count>
   e5f9d:	48 83 7c 24 30 ff                               	cmpq   $0xffffffffffffffff,0x30(%rsp)
   e5fa3:	0f 85 a3 01 00 00                               	jne    e614c <emuella_j2k_codestream::write_component_packet_header+0xefc>
   e5fa9:	85 ed                                           	test   %ebp,%ebp
   e5fab:	4c 8b ac 24 b0 00 00 00                         	mov    0xb0(%rsp),%r13
   e5fb3:	0f 84 89 03 00 00                               	je     e6342 <emuella_j2k_codestream::write_component_packet_header+0x10f2>
   e5fb9:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
   e5fc1:	4c 8b 60 e8                                     	mov    -0x18(%rax),%r12
   e5fc5:	0f bd c5                                        	bsr    %ebp,%eax
   e5fc8:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   e5fcd:	83 f0 1f                                        	xor    $0x1f,%eax
   e5fd0:	41 b7 22                                        	mov    $0x22,%r15b
   e5fd3:	41 28 c7                                        	sub    %al,%r15b
   e5fd6:	4c 89 e0                                        	mov    %r12,%rax
   e5fd9:	44 89 f9                                        	mov    %r15d,%ecx
   e5fdc:	48 d3 e8                                        	shr    %cl,%rax
   e5fdf:	48 85 c0                                        	test   %rax,%rax
   e5fe2:	74 61                                           	je     e6045 <emuella_j2k_codestream::write_component_packet_header+0xdf5>
   e5fe4:	40 b5 fd                                        	mov    $0xfd,%bpl
   e5fe7:	41 b7 04                                        	mov    $0x4,%r15b
   e5fea:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e5fef:	4c 89 ee                                        	mov    %r13,%rsi
   e5ff2:	ba 01 00 00 00                                  	mov    $0x1,%edx
   e5ff7:	e8 c4 6d 06 00                                  	call   14cdc0 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
   e5ffc:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
   e6001:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   e6005:	0f 85 0e 02 00 00                               	jne    e6219 <emuella_j2k_codestream::write_component_packet_header+0xfc9>
   e600b:	45 84 ff                                        	test   %r15b,%r15b
   e600e:	0f 84 18 03 00 00                               	je     e632c <emuella_j2k_codestream::write_component_packet_header+0x10dc>
   e6014:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   e6019:	42 8d 0c 38                                     	lea    (%rax,%r15,1),%ecx
   e601d:	4c 89 e0                                        	mov    %r12,%rax
   e6020:	31 d2                                           	xor    %edx,%edx
   e6022:	48 0f ad d0                                     	shrd   %cl,%rdx,%rax
   e6026:	f6 c1 40                                        	test   $0x40,%cl
   e6029:	48 0f 45 c2                                     	cmovne %rdx,%rax
   e602d:	40 fe cd                                        	dec    %bpl
   e6030:	41 fe c7                                        	inc    %r15b
   e6033:	48 85 c0                                        	test   %rax,%rax
   e6036:	75 b2                                           	jne    e5fea <emuella_j2k_codestream::write_component_packet_header+0xd9a>
   e6038:	4c 8b 7c 24 18                                  	mov    0x18(%rsp),%r15
   e603d:	41 28 ef                                        	sub    %bpl,%r15b
   e6040:	40 f6 dd                                        	neg    %bpl
   e6043:	eb 03                                           	jmp    e6048 <emuella_j2k_codestream::write_component_packet_header+0xdf8>
   e6045:	40 b5 03                                        	mov    $0x3,%bpl
   e6048:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e604d:	4c 89 ee                                        	mov    %r13,%rsi
   e6050:	31 d2                                           	xor    %edx,%edx
   e6052:	e8 69 6d 06 00                                  	call   14cdc0 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
   e6057:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
   e605c:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   e6060:	0f 85 b3 01 00 00                               	jne    e6219 <emuella_j2k_codestream::write_component_packet_header+0xfc9>
   e6066:	4d 89 fd                                        	mov    %r15,%r13
   e6069:	41 38 ef                                        	cmp    %bpl,%r15b
   e606c:	4c 8b bc 24 b0 00 00 00                         	mov    0xb0(%rsp),%r15
   e6074:	0f 82 e7 02 00 00                               	jb     e6361 <emuella_j2k_codestream::write_component_packet_header+0x1111>
   e607a:	4c 89 e0                                        	mov    %r12,%rax
   e607d:	48 c1 e8 20                                     	shr    $0x20,%rax
   e6081:	0f 85 94 02 00 00                               	jne    e631b <emuella_j2k_codestream::write_component_packet_header+0x10cb>
   e6087:	41 8d 6d ff                                     	lea    -0x1(%r13),%ebp
   e608b:	40 0f b6 c5                                     	movzbl %bpl,%eax
   e608f:	31 d2                                           	xor    %edx,%edx
   e6091:	41 0f a3 c4                                     	bt     %eax,%r12d
   e6095:	0f 92 c2                                        	setb   %dl
   e6098:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e609d:	4c 89 fe                                        	mov    %r15,%rsi
   e60a0:	e8 1b 6d 06 00                                  	call   14cdc0 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
   e60a5:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
   e60aa:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   e60ae:	0f 85 13 02 00 00                               	jne    e62c7 <emuella_j2k_codestream::write_component_packet_header+0x1077>
   e60b4:	40 80 c5 ff                                     	add    $0xff,%bpl
   e60b8:	72 d1                                           	jb     e608b <emuella_j2k_codestream::write_component_packet_header+0xe3b>
   e60ba:	44 88 6c 24 28                                  	mov    %r13b,0x28(%rsp)
   e60bf:	e9 1b fe ff ff                                  	jmp    e5edf <emuella_j2k_codestream::write_component_packet_header+0xc8f>
   e60c4:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e60ca:	74 09                                           	je     e60d5 <emuella_j2k_codestream::write_component_packet_header+0xe85>
   e60cc:	48 89 ef                                        	mov    %rbp,%rdi
   e60cf:	ff 15 f3 ec 18 00                               	call   *0x18ecf3(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e60d5:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e60da:	0f 10 84 24 a0 01 00 00                         	movups 0x1a0(%rsp),%xmm0
   e60e2:	0f 29 84 24 80 01 00 00                         	movaps %xmm0,0x180(%rsp)
   e60ea:	48 8b 84 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rax
   e60f2:	48 89 84 24 90 01 00 00                         	mov    %rax,0x190(%rsp)
   e60fa:	48 8b 4c 24 78                                  	mov    0x78(%rsp),%rcx
   e60ff:	48 89 41 18                                     	mov    %rax,0x18(%rcx)
   e6103:	0f 11 41 08                                     	movups %xmm0,0x8(%rcx)
   e6107:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e6111:	48 89 01                                        	mov    %rax,(%rcx)
   e6114:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e6118:	4c 8b 25 a9 ec 18 00                            	mov    0x18eca9(%rip),%r12        # 274dc8 <free@GLIBC_2.2.5>
   e611f:	eb 1c                                           	jmp    e613d <emuella_j2k_codestream::write_component_packet_header+0xeed>
   e6121:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e6130:	49 83 c7 20                                     	add    $0x20,%r15
   e6134:	49 ff ce                                        	dec    %r14
   e6137:	0f 84 b9 00 00 00                               	je     e61f6 <emuella_j2k_codestream::write_component_packet_header+0xfa6>
   e613d:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e6142:	74 ec                                           	je     e6130 <emuella_j2k_codestream::write_component_packet_header+0xee0>
   e6144:	49 8b 3f                                        	mov    (%r15),%rdi
   e6147:	41 ff d4                                        	call   *%r12
   e614a:	eb e4                                           	jmp    e6130 <emuella_j2k_codestream::write_component_packet_header+0xee0>
   e614c:	0f 10 44 24 30                                  	movups 0x30(%rsp),%xmm0
   e6151:	0f 10 4c 24 40                                  	movups 0x40(%rsp),%xmm1
   e6156:	0f 10 54 24 50                                  	movups 0x50(%rsp),%xmm2
   e615b:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   e6160:	0f 11 50 20                                     	movups %xmm2,0x20(%rax)
   e6164:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
   e6168:	0f 11 00                                        	movups %xmm0,(%rax)
   e616b:	e9 1b 01 00 00                                  	jmp    e628b <emuella_j2k_codestream::write_component_packet_header+0x103b>
   e6170:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   e6175:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
   e617c:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
   e6181:	48 85 ed                                        	test   %rbp,%rbp
   e6184:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e6189:	74 2d                                           	je     e61b8 <emuella_j2k_codestream::write_component_packet_header+0xf68>
   e618b:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
   e6193:	4c 8d 78 08                                     	lea    0x8(%rax),%r15
   e6197:	4c 8b 25 2a ec 18 00                            	mov    0x18ec2a(%rip),%r12        # 274dc8 <free@GLIBC_2.2.5>
   e619e:	eb 09                                           	jmp    e61a9 <emuella_j2k_codestream::write_component_packet_header+0xf59>
   e61a0:	49 83 c7 20                                     	add    $0x20,%r15
   e61a4:	48 ff cd                                        	dec    %rbp
   e61a7:	74 0f                                           	je     e61b8 <emuella_j2k_codestream::write_component_packet_header+0xf68>
   e61a9:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e61ae:	74 f0                                           	je     e61a0 <emuella_j2k_codestream::write_component_packet_header+0xf50>
   e61b0:	49 8b 3f                                        	mov    (%r15),%rdi
   e61b3:	41 ff d4                                        	call   *%r12
   e61b6:	eb e8                                           	jmp    e61a0 <emuella_j2k_codestream::write_component_packet_header+0xf50>
   e61b8:	48 83 bc 24 40 01 00 00 00                      	cmpq   $0x0,0x140(%rsp)
   e61c1:	74 0e                                           	je     e61d1 <emuella_j2k_codestream::write_component_packet_header+0xf81>
   e61c3:	48 8b bc 24 80 00 00 00                         	mov    0x80(%rsp),%rdi
   e61cb:	ff 15 f7 eb 18 00                               	call   *0x18ebf7(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e61d1:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e61d5:	4c 8b 25 ec eb 18 00                            	mov    0x18ebec(%rip),%r12        # 274dc8 <free@GLIBC_2.2.5>
   e61dc:	eb 09                                           	jmp    e61e7 <emuella_j2k_codestream::write_component_packet_header+0xf97>
   e61de:	49 83 c7 20                                     	add    $0x20,%r15
   e61e2:	49 ff ce                                        	dec    %r14
   e61e5:	74 0f                                           	je     e61f6 <emuella_j2k_codestream::write_component_packet_header+0xfa6>
   e61e7:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e61ec:	74 f0                                           	je     e61de <emuella_j2k_codestream::write_component_packet_header+0xf8e>
   e61ee:	49 8b 3f                                        	mov    (%r15),%rdi
   e61f1:	41 ff d4                                        	call   *%r12
   e61f4:	eb e8                                           	jmp    e61de <emuella_j2k_codestream::write_component_packet_header+0xf8e>
   e61f6:	4d 85 ed                                        	test   %r13,%r13
   e61f9:	0f 84 9d f6 ff ff                               	je     e589c <emuella_j2k_codestream::write_component_packet_header+0x64c>
   e61ff:	48 89 df                                        	mov    %rbx,%rdi
   e6202:	48 81 c4 b8 01 00 00                            	add    $0x1b8,%rsp
   e6209:	5b                                              	pop    %rbx
   e620a:	41 5c                                           	pop    %r12
   e620c:	41 5d                                           	pop    %r13
   e620e:	41 5e                                           	pop    %r14
   e6210:	41 5f                                           	pop    %r15
   e6212:	5d                                              	pop    %rbp
   e6213:	ff 25 af eb 18 00                               	jmp    *0x18ebaf(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e6219:	0f b6 74 24 38                                  	movzbl 0x38(%rsp),%esi
   e621e:	0f b6 4c 24 3f                                  	movzbl 0x3f(%rsp),%ecx
   e6223:	c1 e1 10                                        	shl    $0x10,%ecx
   e6226:	0f b7 54 24 3d                                  	movzwl 0x3d(%rsp),%edx
   e622b:	09 ca                                           	or     %ecx,%edx
   e622d:	48 c1 e2 20                                     	shl    $0x20,%rdx
   e6231:	8b 4c 24 39                                     	mov    0x39(%rsp),%ecx
   e6235:	48 09 d1                                        	or     %rdx,%rcx
   e6238:	0f 10 44 24 40                                  	movups 0x40(%rsp),%xmm0
   e623d:	0f 29 84 24 60 01 00 00                         	movaps %xmm0,0x160(%rsp)
   e6245:	0f 10 44 24 50                                  	movups 0x50(%rsp),%xmm0
   e624a:	0f 29 84 24 70 01 00 00                         	movaps %xmm0,0x170(%rsp)
   e6252:	48 8b 54 24 78                                  	mov    0x78(%rsp),%rdx
   e6257:	48 89 02                                        	mov    %rax,(%rdx)
   e625a:	40 88 72 08                                     	mov    %sil,0x8(%rdx)
   e625e:	89 4a 09                                        	mov    %ecx,0x9(%rdx)
   e6261:	48 89 c8                                        	mov    %rcx,%rax
   e6264:	48 c1 e8 30                                     	shr    $0x30,%rax
   e6268:	88 42 0f                                        	mov    %al,0xf(%rdx)
   e626b:	48 c1 e9 20                                     	shr    $0x20,%rcx
   e626f:	66 89 4a 0d                                     	mov    %cx,0xd(%rdx)
   e6273:	0f 28 84 24 60 01 00 00                         	movaps 0x160(%rsp),%xmm0
   e627b:	0f 28 8c 24 70 01 00 00                         	movaps 0x170(%rsp),%xmm1
   e6283:	0f 11 42 10                                     	movups %xmm0,0x10(%rdx)
   e6287:	0f 11 4a 20                                     	movups %xmm1,0x20(%rdx)
   e628b:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e6290:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
   e6295:	48 85 ed                                        	test   %rbp,%rbp
   e6298:	74 5f                                           	je     e62f9 <emuella_j2k_codestream::write_component_packet_header+0x10a9>
   e629a:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
   e62a2:	4c 8d 78 08                                     	lea    0x8(%rax),%r15
   e62a6:	4c 8b 25 1b eb 18 00                            	mov    0x18eb1b(%rip),%r12        # 274dc8 <free@GLIBC_2.2.5>
   e62ad:	eb 09                                           	jmp    e62b8 <emuella_j2k_codestream::write_component_packet_header+0x1068>
   e62af:	49 83 c7 20                                     	add    $0x20,%r15
   e62b3:	48 ff cd                                        	dec    %rbp
   e62b6:	74 41                                           	je     e62f9 <emuella_j2k_codestream::write_component_packet_header+0x10a9>
   e62b8:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e62bd:	74 f0                                           	je     e62af <emuella_j2k_codestream::write_component_packet_header+0x105f>
   e62bf:	49 8b 3f                                        	mov    (%r15),%rdi
   e62c2:	41 ff d4                                        	call   *%r12
   e62c5:	eb e8                                           	jmp    e62af <emuella_j2k_codestream::write_component_packet_header+0x105f>
   e62c7:	48 8b 4c 24 58                                  	mov    0x58(%rsp),%rcx
   e62cc:	48 8b 54 24 78                                  	mov    0x78(%rsp),%rdx
   e62d1:	48 89 4a 28                                     	mov    %rcx,0x28(%rdx)
   e62d5:	0f 10 44 24 38                                  	movups 0x38(%rsp),%xmm0
   e62da:	0f 10 4c 24 48                                  	movups 0x48(%rsp),%xmm1
   e62df:	0f 11 4a 18                                     	movups %xmm1,0x18(%rdx)
   e62e3:	0f 11 42 08                                     	movups %xmm0,0x8(%rdx)
   e62e7:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
   e62ec:	48 89 02                                        	mov    %rax,(%rdx)
   e62ef:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e62f4:	48 85 ed                                        	test   %rbp,%rbp
   e62f7:	75 a1                                           	jne    e629a <emuella_j2k_codestream::write_component_packet_header+0x104a>
   e62f9:	48 83 bc 24 40 01 00 00 00                      	cmpq   $0x0,0x140(%rsp)
   e6302:	0f 84 0c fe ff ff                               	je     e6114 <emuella_j2k_codestream::write_component_packet_header+0xec4>
   e6308:	48 8b bc 24 80 00 00 00                         	mov    0x80(%rsp),%rdi
   e6310:	ff 15 b2 ea 18 00                               	call   *0x18eab2(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e6316:	e9 f9 fd ff ff                                  	jmp    e6114 <emuella_j2k_codestream::write_component_packet_header+0xec4>
   e631b:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e6325:	48 8b 54 24 78                                  	mov    0x78(%rsp),%rdx
   e632a:	eb bb                                           	jmp    e62e7 <emuella_j2k_codestream::write_component_packet_header+0x1097>
   e632c:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e6336:	31 c9                                           	xor    %ecx,%ecx
   e6338:	0f b6 74 24 28                                  	movzbl 0x28(%rsp),%esi
   e633d:	e9 10 ff ff ff                                  	jmp    e6252 <emuella_j2k_codestream::write_component_packet_header+0x1002>
   e6342:	48 8d 3d 47 86 18 00                            	lea    0x188647(%rip),%rdi        # 26e990 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0xd18>
   e6349:	ff 15 11 ee 18 00                               	call   *0x18ee11(%rip)        # 275160 <_DYNAMIC+0x5b0>
   e634f:	eb 4a                                           	jmp    e639b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e6351:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e6356:	4c 89 ee                                        	mov    %r13,%rsi
   e6359:	ff 15 a1 ea 18 00                               	call   *0x18eaa1(%rip)        # 274e00 <_DYNAMIC+0x250>
   e635f:	eb 3a                                           	jmp    e639b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e6361:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e636b:	eb cb                                           	jmp    e6338 <emuella_j2k_codestream::write_component_packet_header+0x10e8>
   e636d:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e6372:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
   e6377:	ff 15 83 ea 18 00                               	call   *0x18ea83(%rip)        # 274e00 <_DYNAMIC+0x250>
   e637d:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e6382:	4c 89 e6                                        	mov    %r12,%rsi
   e6385:	ff 15 75 ea 18 00                               	call   *0x18ea75(%rip)        # 274e00 <_DYNAMIC+0x250>
   e638b:	eb 0e                                           	jmp    e639b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e638d:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e6392:	4c 89 fe                                        	mov    %r15,%rsi
   e6395:	ff 15 65 ea 18 00                               	call   *0x18ea65(%rip)        # 274e00 <_DYNAMIC+0x250>
   e639b:	0f 0b                                           	ud2
   e639d:	e9 a2 00 00 00                                  	jmp    e6444 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e63a2:	e9 9d 00 00 00                                  	jmp    e6444 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e63a7:	e9 98 00 00 00                                  	jmp    e6444 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e63ac:	49 89 c4                                        	mov    %rax,%r12
   e63af:	4d 85 ed                                        	test   %r13,%r13
   e63b2:	0f 84 e1 00 00 00                               	je     e6499 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e63b8:	4c 89 ff                                        	mov    %r15,%rdi
   e63bb:	e9 d3 00 00 00                                  	jmp    e6493 <emuella_j2k_codestream::write_component_packet_header+0x1243>
   e63c0:	49 89 c4                                        	mov    %rax,%r12
   e63c3:	e9 fd 00 00 00                                  	jmp    e64c5 <emuella_j2k_codestream::write_component_packet_header+0x1275>
   e63c8:	eb 7a                                           	jmp    e6444 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e63ca:	49 89 c5                                        	mov    %rax,%r13
   e63cd:	4d 85 ff                                        	test   %r15,%r15
   e63d0:	75 08                                           	jne    e63da <emuella_j2k_codestream::write_component_packet_header+0x118a>
   e63d2:	4d 89 ec                                        	mov    %r13,%r12
   e63d5:	e9 bf 00 00 00                                  	jmp    e6499 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e63da:	4c 89 e7                                        	mov    %r12,%rdi
   e63dd:	ff 15 e5 e9 18 00                               	call   *0x18e9e5(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e63e3:	4d 89 ec                                        	mov    %r13,%r12
   e63e6:	e9 ae 00 00 00                                  	jmp    e6499 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e63eb:	49 89 c4                                        	mov    %rax,%r12
   e63ee:	e9 a6 00 00 00                                  	jmp    e6499 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e63f3:	49 89 c4                                        	mov    %rax,%r12
   e63f6:	4d 85 ff                                        	test   %r15,%r15
   e63f9:	0f 84 50 01 00 00                               	je     e654f <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e63ff:	4c 89 f7                                        	mov    %r14,%rdi
   e6402:	e9 42 01 00 00                                  	jmp    e6549 <emuella_j2k_codestream::write_component_packet_header+0x12f9>
   e6407:	49 89 c4                                        	mov    %rax,%r12
   e640a:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e6413:	0f 84 a7 00 00 00                               	je     e64c0 <emuella_j2k_codestream::write_component_packet_header+0x1270>
   e6419:	48 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%rdi
   e6421:	e9 94 00 00 00                                  	jmp    e64ba <emuella_j2k_codestream::write_component_packet_header+0x126a>
   e6426:	49 89 c4                                        	mov    %rax,%r12
   e6429:	4d 85 ff                                        	test   %r15,%r15
   e642c:	0f 84 1d 01 00 00                               	je     e654f <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e6432:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
   e6437:	e9 0d 01 00 00                                  	jmp    e6549 <emuella_j2k_codestream::write_component_packet_header+0x12f9>
   e643c:	49 89 c4                                        	mov    %rax,%r12
   e643f:	e9 0b 01 00 00                                  	jmp    e654f <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e6444:	49 89 c4                                        	mov    %rax,%r12
   e6447:	48 8d bc 24 40 01 00 00                         	lea    0x140(%rsp),%rdi
   e644f:	e8 bc 2e fb ff                                  	call   99310 <core::ptr::drop_glue::<emuella_j2k_codestream::EncTagTree>>
   e6454:	eb 6a                                           	jmp    e64c0 <emuella_j2k_codestream::write_component_packet_header+0x1270>
   e6456:	49 89 c4                                        	mov    %rax,%r12
   e6459:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e6462:	0f 84 07 01 00 00                               	je     e656f <emuella_j2k_codestream::write_component_packet_header+0x131f>
   e6468:	48 8b 9c 24 98 00 00 00                         	mov    0x98(%rsp),%rbx
   e6470:	e9 f1 00 00 00                                  	jmp    e6566 <emuella_j2k_codestream::write_component_packet_header+0x1316>
   e6475:	eb 0c                                           	jmp    e6483 <emuella_j2k_codestream::write_component_packet_header+0x1233>
   e6477:	eb 0a                                           	jmp    e6483 <emuella_j2k_codestream::write_component_packet_header+0x1233>
   e6479:	e9 bb 00 00 00                                  	jmp    e6539 <emuella_j2k_codestream::write_component_packet_header+0x12e9>
   e647e:	e9 b6 00 00 00                                  	jmp    e6539 <emuella_j2k_codestream::write_component_packet_header+0x12e9>
   e6483:	49 89 c4                                        	mov    %rax,%r12
   e6486:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   e648c:	74 0b                                           	je     e6499 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e648e:	48 8b 7c 24 38                                  	mov    0x38(%rsp),%rdi
   e6493:	ff 15 2f e9 18 00                               	call   *0x18e92f(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e6499:	48 8b 44 24 68                                  	mov    0x68(%rsp),%rax
   e649e:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   e64a3:	4c 8b 6c 24 70                                  	mov    0x70(%rsp),%r13
   e64a8:	4d 85 ed                                        	test   %r13,%r13
   e64ab:	75 52                                           	jne    e64ff <emuella_j2k_codestream::write_component_packet_header+0x12af>
   e64ad:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e64b3:	74 0b                                           	je     e64c0 <emuella_j2k_codestream::write_component_packet_header+0x1270>
   e64b5:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
   e64ba:	ff 15 08 e9 18 00                               	call   *0x18e908(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e64c0:	48 8b 6c 24 20                                  	mov    0x20(%rsp),%rbp
   e64c5:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e64c9:	4c 8b 2d f8 e8 18 00                            	mov    0x18e8f8(%rip),%r13        # 274dc8 <free@GLIBC_2.2.5>
   e64d0:	eb 17                                           	jmp    e64e9 <emuella_j2k_codestream::write_component_packet_header+0x1299>
   e64d2:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e64e0:	49 83 c7 20                                     	add    $0x20,%r15
   e64e4:	49 ff ce                                        	dec    %r14
   e64e7:	74 0f                                           	je     e64f8 <emuella_j2k_codestream::write_component_packet_header+0x12a8>
   e64e9:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e64ee:	74 f0                                           	je     e64e0 <emuella_j2k_codestream::write_component_packet_header+0x1290>
   e64f0:	49 8b 3f                                        	mov    (%r15),%rdi
   e64f3:	41 ff d5                                        	call   *%r13
   e64f6:	eb e8                                           	jmp    e64e0 <emuella_j2k_codestream::write_component_packet_header+0x1290>
   e64f8:	48 85 ed                                        	test   %rbp,%rbp
   e64fb:	75 69                                           	jne    e6566 <emuella_j2k_codestream::write_component_packet_header+0x1316>
   e64fd:	eb 70                                           	jmp    e656f <emuella_j2k_codestream::write_component_packet_header+0x131f>
   e64ff:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   e6504:	48 8d 68 08                                     	lea    0x8(%rax),%rbp
   e6508:	4c 8b 3d b9 e8 18 00                            	mov    0x18e8b9(%rip),%r15        # 274dc8 <free@GLIBC_2.2.5>
   e650f:	eb 18                                           	jmp    e6529 <emuella_j2k_codestream::write_component_packet_header+0x12d9>
   e6511:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e6520:	48 83 c5 20                                     	add    $0x20,%rbp
   e6524:	49 ff cd                                        	dec    %r13
   e6527:	74 84                                           	je     e64ad <emuella_j2k_codestream::write_component_packet_header+0x125d>
   e6529:	48 83 7d f8 00                                  	cmpq   $0x0,-0x8(%rbp)
   e652e:	74 f0                                           	je     e6520 <emuella_j2k_codestream::write_component_packet_header+0x12d0>
   e6530:	48 8b 7d 00                                     	mov    0x0(%rbp),%rdi
   e6534:	41 ff d7                                        	call   *%r15
   e6537:	eb e7                                           	jmp    e6520 <emuella_j2k_codestream::write_component_packet_header+0x12d0>
   e6539:	49 89 c4                                        	mov    %rax,%r12
   e653c:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   e6542:	74 0b                                           	je     e654f <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e6544:	48 8b 7c 24 38                                  	mov    0x38(%rsp),%rdi
   e6549:	ff 15 79 e8 18 00                               	call   *0x18e879(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e654f:	48 8b 5c 24 68                                  	mov    0x68(%rsp),%rbx
   e6554:	4c 8b 74 24 70                                  	mov    0x70(%rsp),%r14
   e6559:	4d 85 f6                                        	test   %r14,%r14
   e655c:	75 19                                           	jne    e6577 <emuella_j2k_codestream::write_component_packet_header+0x1327>
   e655e:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e6564:	74 09                                           	je     e656f <emuella_j2k_codestream::write_component_packet_header+0x131f>
   e6566:	48 89 df                                        	mov    %rbx,%rdi
   e6569:	ff 15 59 e8 18 00                               	call   *0x18e859(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   e656f:	4c 89 e7                                        	mov    %r12,%rdi
   e6572:	e8 29 5c 18 00                                  	call   26c1a0 <_Unwind_Resume@plt>
   e6577:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e657b:	4c 8b 2d 46 e8 18 00                            	mov    0x18e846(%rip),%r13        # 274dc8 <free@GLIBC_2.2.5>
   e6582:	eb 15                                           	jmp    e6599 <emuella_j2k_codestream::write_component_packet_header+0x1349>
   e6584:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
   e6590:	49 83 c7 20                                     	add    $0x20,%r15
   e6594:	49 ff ce                                        	dec    %r14
   e6597:	74 c5                                           	je     e655e <emuella_j2k_codestream::write_component_packet_header+0x130e>
   e6599:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e659e:	74 f0                                           	je     e6590 <emuella_j2k_codestream::write_component_packet_header+0x1340>
   e65a0:	49 8b 3f                                        	mov    (%r15),%rdi
   e65a3:	41 ff d5                                        	call   *%r13
   e65a6:	eb e8                                           	jmp    e6590 <emuella_j2k_codestream::write_component_packet_header+0x1340>
