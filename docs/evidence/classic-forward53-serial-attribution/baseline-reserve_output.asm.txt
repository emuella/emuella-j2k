Disassembly of section .text:

00000000001501f0 <emuella_j2k_codestream::scalable_lossless::reserve_output>:
  1501f0:	4c 8b 46 10                                     	mov    0x10(%rsi),%r8
  1501f4:	4c 01 c2                                        	add    %r8,%rdx
  1501f7:	0f 82 21 01 00 00                               	jb     15031e <emuella_j2k_codestream::scalable_lossless::reserve_output+0x12e>
  1501fd:	48 39 ca                                        	cmp    %rcx,%rdx
  150200:	76 33                                           	jbe    150235 <emuella_j2k_codestream::scalable_lossless::reserve_output+0x45>
  150202:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  15020c:	48 89 07                                        	mov    %rax,(%rdi)
  15020f:	48 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%rdi)
  150217:	48 8d 05 de da ec ff                            	lea    -0x132522(%rip),%rax        # 1dcfc <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x45c>
  15021e:	48 89 47 18                                     	mov    %rax,0x18(%rdi)
  150222:	48 c7 47 20 32 00 00 00                         	movq   $0x32,0x20(%rdi)
  15022a:	66 c7 47 28 04 00                               	movw   $0x4,0x28(%rdi)
  150230:	c6 47 2c 0a                                     	movb   $0xa,0x2c(%rdi)
  150234:	c3                                              	ret
  150235:	41 57                                           	push   %r15
  150237:	41 56                                           	push   %r14
  150239:	41 54                                           	push   %r12
  15023b:	53                                              	push   %rbx
  15023c:	48 83 ec 18                                     	sub    $0x18,%rsp
  150240:	48 8b 06                                        	mov    (%rsi),%rax
  150243:	48 39 c2                                        	cmp    %rax,%rdx
  150246:	0f 86 bf 00 00 00                               	jbe    15030b <emuella_j2k_codestream::scalable_lossless::reserve_output+0x11b>
  15024c:	48 8d 1c 00                                     	lea    (%rax,%rax,1),%rbx
  150250:	48 39 d9                                        	cmp    %rbx,%rcx
  150253:	48 0f 42 d9                                     	cmovb  %rcx,%rbx
  150257:	48 39 d3                                        	cmp    %rdx,%rbx
  15025a:	48 0f 46 da                                     	cmovbe %rdx,%rbx
  15025e:	48 89 da                                        	mov    %rbx,%rdx
  150261:	4c 29 c2                                        	sub    %r8,%rdx
  150264:	49 89 c1                                        	mov    %rax,%r9
  150267:	4d 29 c1                                        	sub    %r8,%r9
  15026a:	4c 39 ca                                        	cmp    %r9,%rdx
  15026d:	77 40                                           	ja     1502af <emuella_j2k_codestream::scalable_lossless::reserve_output+0xbf>
  15026f:	48 89 c3                                        	mov    %rax,%rbx
  150272:	48 39 cb                                        	cmp    %rcx,%rbx
  150275:	0f 86 90 00 00 00                               	jbe    15030b <emuella_j2k_codestream::scalable_lossless::reserve_output+0x11b>
  15027b:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  150285:	48 89 07                                        	mov    %rax,(%rdi)
  150288:	48 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%rdi)
  150290:	48 8d 05 30 da ec ff                            	lea    -0x1325d0(%rip),%rax        # 1dcc7 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x427>
  150297:	48 89 47 18                                     	mov    %rax,0x18(%rdi)
  15029b:	48 c7 47 20 35 00 00 00                         	movq   $0x35,0x20(%rdi)
  1502a3:	66 c7 47 28 04 00                               	movw   $0x4,0x28(%rdi)
  1502a9:	c6 47 2c 0a                                     	movb   $0xa,0x2c(%rdi)
  1502ad:	eb 63                                           	jmp    150312 <emuella_j2k_codestream::scalable_lossless::reserve_output+0x122>
  1502af:	49 89 cf                                        	mov    %rcx,%r15
  1502b2:	49 89 fe                                        	mov    %rdi,%r14
  1502b5:	49 89 f4                                        	mov    %rsi,%r12
  1502b8:	48 8b 56 08                                     	mov    0x8(%rsi),%rdx
  1502bc:	48 89 e7                                        	mov    %rsp,%rdi
  1502bf:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
  1502c5:	41 b9 01 00 00 00                               	mov    $0x1,%r9d
  1502cb:	48 89 c6                                        	mov    %rax,%rsi
  1502ce:	48 89 d9                                        	mov    %rbx,%rcx
  1502d1:	e8 ca 96 ff ff                                  	call   1499a0 <<alloc::raw_vec::RawVecInner>::finish_grow>
  1502d6:	83 3c 24 01                                     	cmpl   $0x1,(%rsp)
  1502da:	75 12                                           	jne    1502ee <emuella_j2k_codestream::scalable_lossless::reserve_output+0xfe>
  1502dc:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  1502e6:	48 ff c0                                        	inc    %rax
  1502e9:	49 89 06                                        	mov    %rax,(%r14)
  1502ec:	eb 24                                           	jmp    150312 <emuella_j2k_codestream::scalable_lossless::reserve_output+0x122>
  1502ee:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  1502f3:	49 89 44 24 08                                  	mov    %rax,0x8(%r12)
  1502f8:	49 89 1c 24                                     	mov    %rbx,(%r12)
  1502fc:	4c 89 f7                                        	mov    %r14,%rdi
  1502ff:	4c 89 f9                                        	mov    %r15,%rcx
  150302:	48 39 cb                                        	cmp    %rcx,%rbx
  150305:	0f 87 70 ff ff ff                               	ja     15027b <emuella_j2k_codestream::scalable_lossless::reserve_output+0x8b>
  15030b:	48 c7 07 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rdi)
  150312:	48 83 c4 18                                     	add    $0x18,%rsp
  150316:	5b                                              	pop    %rbx
  150317:	41 5c                                           	pop    %r12
  150319:	41 5e                                           	pop    %r14
  15031b:	41 5f                                           	pop    %r15
  15031d:	c3                                              	ret
  15031e:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  150328:	48 ff c0                                        	inc    %rax
  15032b:	48 89 07                                        	mov    %rax,(%rdi)
  15032e:	c3                                              	ret
