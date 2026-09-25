Disassembly of section .text:

00000000000d6240 <emuella_j2k_codestream::write_native_main_header>:
   d6240:	55                                              	push   %rbp
   d6241:	41 57                                           	push   %r15
   d6243:	41 56                                           	push   %r14
   d6245:	41 55                                           	push   %r13
   d6247:	41 54                                           	push   %r12
   d6249:	53                                              	push   %rbx
   d624a:	48 83 ec 28                                     	sub    $0x28,%rsp
   d624e:	44 89 4c 24 24                                  	mov    %r9d,0x24(%rsp)
   d6253:	44 89 44 24 20                                  	mov    %r8d,0x20(%rsp)
   d6258:	89 4c 24 1c                                     	mov    %ecx,0x1c(%rsp)
   d625c:	89 54 24 18                                     	mov    %edx,0x18(%rsp)
   d6260:	49 89 f4                                        	mov    %rsi,%r12
   d6263:	48 89 7c 24 10                                  	mov    %rdi,0x10(%rsp)
   d6268:	0f b7 5c 24 68                                  	movzwl 0x68(%rsp),%ebx
   d626d:	0f b6 84 24 88 00 00 00                         	movzbl 0x88(%rsp),%eax
   d6275:	88 44 24 07                                     	mov    %al,0x7(%rsp)
   d6279:	48 8b ac 24 80 00 00 00                         	mov    0x80(%rsp),%rbp
   d6281:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   d6286:	48 89 44 24 08                                  	mov    %rax,0x8(%rsp)
   d628b:	0f b6 44 24 70                                  	movzbl 0x70(%rsp),%eax
   d6290:	88 44 24 06                                     	mov    %al,0x6(%rsp)
   d6294:	44 0f b6 7c 24 60                               	movzbl 0x60(%rsp),%r15d
   d629a:	85 db                                           	test   %ebx,%ebx
   d629c:	74 33                                           	je     d62d1 <emuella_j2k_codestream::write_native_main_header+0x91>
   d629e:	44 8d 2c 9d 00 00 00 00                         	lea    0x0(,%rbx,4),%r13d
   d62a6:	4c 89 ef                                        	mov    %r13,%rdi
   d62a9:	ff 15 39 7b 19 00                               	call   *0x197b39(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   d62af:	48 85 c0                                        	test   %rax,%rax
   d62b2:	0f 84 4a 09 00 00                               	je     d6c02 <emuella_j2k_codestream::write_native_main_header+0x9c2>
   d62b8:	49 89 c6                                        	mov    %rax,%r14
   d62bb:	41 0f b6 c7                                     	movzbl %r15b,%eax
   d62bf:	c1 e0 08                                        	shl    $0x8,%eax
   d62c2:	0d 00 00 01 01                                  	or     $0x1010000,%eax
   d62c7:	66 83 fb 08                                     	cmp    $0x8,%bx
   d62cb:	73 0c                                           	jae    d62d9 <emuella_j2k_codestream::write_native_main_header+0x99>
   d62cd:	31 c9                                           	xor    %ecx,%ecx
   d62cf:	eb 4f                                           	jmp    d6320 <emuella_j2k_codestream::write_native_main_header+0xe0>
   d62d1:	41 be 01 00 00 00                               	mov    $0x1,%r14d
   d62d7:	eb 53                                           	jmp    d632c <emuella_j2k_codestream::write_native_main_header+0xec>
   d62d9:	89 d9                                           	mov    %ebx,%ecx
   d62db:	83 e1 f8                                        	and    $0xfffffff8,%ecx
   d62de:	66 0f 6e c0                                     	movd   %eax,%xmm0
   d62e2:	66 0f 70 c0 00                                  	pshufd $0x0,%xmm0,%xmm0
   d62e7:	8d 14 9d 00 00 00 00                            	lea    0x0(,%rbx,4),%edx
   d62ee:	83 e2 e0                                        	and    $0xffffffe0,%edx
   d62f1:	31 f6                                           	xor    %esi,%esi
   d62f3:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   d6300:	f3 41 0f 7f 04 36                               	movdqu %xmm0,(%r14,%rsi,1)
   d6306:	f3 41 0f 7f 44 36 10                            	movdqu %xmm0,0x10(%r14,%rsi,1)
   d630d:	48 83 c6 20                                     	add    $0x20,%rsi
   d6311:	48 39 f2                                        	cmp    %rsi,%rdx
   d6314:	75 ea                                           	jne    d6300 <emuella_j2k_codestream::write_native_main_header+0xc0>
   d6316:	39 d9                                           	cmp    %ebx,%ecx
   d6318:	74 12                                           	je     d632c <emuella_j2k_codestream::write_native_main_header+0xec>
   d631a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
   d6320:	41 89 04 8e                                     	mov    %eax,(%r14,%rcx,4)
   d6324:	48 ff c1                                        	inc    %rcx
   d6327:	48 39 cb                                        	cmp    %rcx,%rbx
   d632a:	75 f4                                           	jne    d6320 <emuella_j2k_codestream::write_native_main_header+0xe0>
   d632c:	48 83 fd 07                                     	cmp    $0x7,%rbp
   d6330:	0f 85 8a 02 00 00                               	jne    d65c0 <emuella_j2k_codestream::write_native_main_header+0x380>
   d6336:	66 b9 03 00                                     	mov    $0x3,%cx
   d633a:	89 d8                                           	mov    %ebx,%eax
   d633c:	66 f7 e1                                        	mul    %cx
   d633f:	0f 80 85 05 00 00                               	jo     d68ca <emuella_j2k_codestream::write_native_main_header+0x68a>
   d6345:	89 c5                                           	mov    %eax,%ebp
   d6347:	66 83 f8 d9                                     	cmp    $0xffd9,%ax
   d634b:	0f 87 9a 05 00 00                               	ja     d68eb <emuella_j2k_codestream::write_native_main_header+0x6ab>
   d6351:	49 8b 04 24                                     	mov    (%r12),%rax
   d6355:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d635a:	4c 29 e8                                        	sub    %r13,%rax
   d635d:	48 83 f8 01                                     	cmp    $0x1,%rax
   d6361:	0f 86 b7 05 00 00                               	jbe    d691e <emuella_j2k_codestream::write_native_main_header+0x6de>
   d6367:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d636c:	66 42 c7 04 28 ff 4f                            	movw   $0x4fff,(%rax,%r13,1)
   d6373:	49 83 c5 02                                     	add    $0x2,%r13
   d6377:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d637c:	49 8b 04 24                                     	mov    (%r12),%rax
   d6380:	4c 29 e8                                        	sub    %r13,%rax
   d6383:	48 83 f8 01                                     	cmp    $0x1,%rax
   d6387:	0f 86 b6 05 00 00                               	jbe    d6943 <emuella_j2k_codestream::write_native_main_header+0x703>
   d638d:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6392:	66 42 c7 04 28 ff 51                            	movw   $0x51ff,(%rax,%r13,1)
   d6399:	49 83 c5 02                                     	add    $0x2,%r13
   d639d:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d63a2:	49 8b 04 24                                     	mov    (%r12),%rax
   d63a6:	4c 29 e8                                        	sub    %r13,%rax
   d63a9:	48 83 f8 01                                     	cmp    $0x1,%rax
   d63ad:	0f 86 b5 05 00 00                               	jbe    d6968 <emuella_j2k_codestream::write_native_main_header+0x728>
   d63b3:	83 c5 26                                        	add    $0x26,%ebp
   d63b6:	66 c1 c5 08                                     	rol    $0x8,%bp
   d63ba:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d63bf:	66 42 89 2c 28                                  	mov    %bp,(%rax,%r13,1)
   d63c4:	49 83 c5 02                                     	add    $0x2,%r13
   d63c8:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d63cd:	49 8b 04 24                                     	mov    (%r12),%rax
   d63d1:	4c 29 e8                                        	sub    %r13,%rax
   d63d4:	48 83 f8 01                                     	cmp    $0x1,%rax
   d63d8:	0f 86 af 05 00 00                               	jbe    d698d <emuella_j2k_codestream::write_native_main_header+0x74d>
   d63de:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d63e3:	66 42 c7 04 28 00 00                            	movw   $0x0,(%rax,%r13,1)
   d63ea:	49 83 c5 02                                     	add    $0x2,%r13
   d63ee:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d63f3:	49 8b 04 24                                     	mov    (%r12),%rax
   d63f7:	4c 29 e8                                        	sub    %r13,%rax
   d63fa:	48 83 f8 03                                     	cmp    $0x3,%rax
   d63fe:	0f 86 ae 05 00 00                               	jbe    d69b2 <emuella_j2k_codestream::write_native_main_header+0x772>
   d6404:	8b 4c 24 18                                     	mov    0x18(%rsp),%ecx
   d6408:	0f c9                                           	bswap  %ecx
   d640a:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d640f:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d6413:	49 83 c5 04                                     	add    $0x4,%r13
   d6417:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d641c:	49 8b 04 24                                     	mov    (%r12),%rax
   d6420:	4c 29 e8                                        	sub    %r13,%rax
   d6423:	48 83 f8 03                                     	cmp    $0x3,%rax
   d6427:	0f 86 aa 05 00 00                               	jbe    d69d7 <emuella_j2k_codestream::write_native_main_header+0x797>
   d642d:	8b 4c 24 1c                                     	mov    0x1c(%rsp),%ecx
   d6431:	0f c9                                           	bswap  %ecx
   d6433:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6438:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d643c:	49 83 c5 04                                     	add    $0x4,%r13
   d6440:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6445:	49 8b 04 24                                     	mov    (%r12),%rax
   d6449:	4c 29 e8                                        	sub    %r13,%rax
   d644c:	48 83 f8 03                                     	cmp    $0x3,%rax
   d6450:	0f 86 a6 05 00 00                               	jbe    d69fc <emuella_j2k_codestream::write_native_main_header+0x7bc>
   d6456:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d645b:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d6463:	49 83 c5 04                                     	add    $0x4,%r13
   d6467:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d646c:	49 8b 04 24                                     	mov    (%r12),%rax
   d6470:	4c 29 e8                                        	sub    %r13,%rax
   d6473:	48 83 f8 03                                     	cmp    $0x3,%rax
   d6477:	0f 86 a4 05 00 00                               	jbe    d6a21 <emuella_j2k_codestream::write_native_main_header+0x7e1>
   d647d:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6482:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d648a:	49 83 c5 04                                     	add    $0x4,%r13
   d648e:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6493:	49 8b 04 24                                     	mov    (%r12),%rax
   d6497:	4c 29 e8                                        	sub    %r13,%rax
   d649a:	48 83 f8 03                                     	cmp    $0x3,%rax
   d649e:	0f 86 a2 05 00 00                               	jbe    d6a46 <emuella_j2k_codestream::write_native_main_header+0x806>
   d64a4:	8b 4c 24 20                                     	mov    0x20(%rsp),%ecx
   d64a8:	0f c9                                           	bswap  %ecx
   d64aa:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d64af:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d64b3:	49 83 c5 04                                     	add    $0x4,%r13
   d64b7:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d64bc:	49 8b 04 24                                     	mov    (%r12),%rax
   d64c0:	4c 29 e8                                        	sub    %r13,%rax
   d64c3:	48 83 f8 03                                     	cmp    $0x3,%rax
   d64c7:	0f 86 9e 05 00 00                               	jbe    d6a6b <emuella_j2k_codestream::write_native_main_header+0x82b>
   d64cd:	8b 4c 24 24                                     	mov    0x24(%rsp),%ecx
   d64d1:	0f c9                                           	bswap  %ecx
   d64d3:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d64d8:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d64dc:	49 83 c5 04                                     	add    $0x4,%r13
   d64e0:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d64e5:	49 8b 04 24                                     	mov    (%r12),%rax
   d64e9:	4c 29 e8                                        	sub    %r13,%rax
   d64ec:	48 83 f8 03                                     	cmp    $0x3,%rax
   d64f0:	0f 86 9a 05 00 00                               	jbe    d6a90 <emuella_j2k_codestream::write_native_main_header+0x850>
   d64f6:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d64fb:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d6503:	49 83 c5 04                                     	add    $0x4,%r13
   d6507:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d650c:	49 8b 04 24                                     	mov    (%r12),%rax
   d6510:	4c 29 e8                                        	sub    %r13,%rax
   d6513:	48 83 f8 03                                     	cmp    $0x3,%rax
   d6517:	0f 86 98 05 00 00                               	jbe    d6ab5 <emuella_j2k_codestream::write_native_main_header+0x875>
   d651d:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6522:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d652a:	49 83 c5 04                                     	add    $0x4,%r13
   d652e:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6533:	49 8b 04 24                                     	mov    (%r12),%rax
   d6537:	4c 29 e8                                        	sub    %r13,%rax
   d653a:	48 83 f8 01                                     	cmp    $0x1,%rax
   d653e:	0f 86 96 05 00 00                               	jbe    d6ada <emuella_j2k_codestream::write_native_main_header+0x89a>
   d6544:	89 d8                                           	mov    %ebx,%eax
   d6546:	66 c1 c0 08                                     	rol    $0x8,%ax
   d654a:	49 8b 4c 24 08                                  	mov    0x8(%r12),%rcx
   d654f:	66 42 89 04 29                                  	mov    %ax,(%rcx,%r13,1)
   d6554:	49 83 c5 02                                     	add    $0x2,%r13
   d6558:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d655d:	45 84 ff                                        	test   %r15b,%r15b
   d6560:	0f 84 25 03 00 00                               	je     d688b <emuella_j2k_codestream::write_native_main_header+0x64b>
   d6566:	66 85 db                                        	test   %bx,%bx
   d6569:	74 75                                           	je     d65e0 <emuella_j2k_codestream::write_native_main_header+0x3a0>
   d656b:	41 fe cf                                        	dec    %r15b
   d656e:	89 dd                                           	mov    %ebx,%ebp
   d6570:	49 8b 04 24                                     	mov    (%r12),%rax
   d6574:	4c 29 e8                                        	sub    %r13,%rax
   d6577:	48 83 f8 02                                     	cmp    $0x2,%rax
   d657b:	76 21                                           	jbe    d659e <emuella_j2k_codestream::write_native_main_header+0x35e>
   d657d:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6582:	46 88 3c 28                                     	mov    %r15b,(%rax,%r13,1)
   d6586:	66 42 c7 44 28 01 01 01                         	movw   $0x101,0x1(%rax,%r13,1)
   d658e:	49 83 c5 03                                     	add    $0x3,%r13
   d6592:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6597:	66 ff cd                                        	dec    %bp
   d659a:	75 d4                                           	jne    d6570 <emuella_j2k_codestream::write_native_main_header+0x330>
   d659c:	eb 42                                           	jmp    d65e0 <emuella_j2k_codestream::write_native_main_header+0x3a0>
   d659e:	ba 03 00 00 00                                  	mov    $0x3,%edx
   d65a3:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d65a8:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d65ae:	4c 89 e7                                        	mov    %r12,%rdi
   d65b1:	4c 89 ee                                        	mov    %r13,%rsi
   d65b4:	e8 37 3f fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d65b9:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d65be:	eb bd                                           	jmp    d657d <emuella_j2k_codestream::write_native_main_header+0x33d>
   d65c0:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d65ca:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d65cf:	48 89 01                                        	mov    %rax,(%rcx)
   d65d2:	66 85 db                                        	test   %bx,%bx
   d65d5:	0f 85 d8 02 00 00                               	jne    d68b3 <emuella_j2k_codestream::write_native_main_header+0x673>
   d65db:	e9 2f 03 00 00                                  	jmp    d690f <emuella_j2k_codestream::write_native_main_header+0x6cf>
   d65e0:	49 8b 04 24                                     	mov    (%r12),%rax
   d65e4:	4c 29 e8                                        	sub    %r13,%rax
   d65e7:	48 83 f8 01                                     	cmp    $0x1,%rax
   d65eb:	0f 86 0e 05 00 00                               	jbe    d6aff <emuella_j2k_codestream::write_native_main_header+0x8bf>
   d65f1:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d65f6:	66 42 c7 04 28 ff 52                            	movw   $0x52ff,(%rax,%r13,1)
   d65fd:	49 83 c5 02                                     	add    $0x2,%r13
   d6601:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6606:	49 8b 04 24                                     	mov    (%r12),%rax
   d660a:	4c 29 e8                                        	sub    %r13,%rax
   d660d:	48 83 f8 01                                     	cmp    $0x1,%rax
   d6611:	0f 86 0d 05 00 00                               	jbe    d6b24 <emuella_j2k_codestream::write_native_main_header+0x8e4>
   d6617:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d661c:	66 42 c7 04 28 00 0c                            	movw   $0xc00,(%rax,%r13,1)
   d6623:	49 83 c5 02                                     	add    $0x2,%r13
   d6627:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d662c:	49 8b 04 24                                     	mov    (%r12),%rax
   d6630:	4c 29 e8                                        	sub    %r13,%rax
   d6633:	48 83 f8 01                                     	cmp    $0x1,%rax
   d6637:	0f 86 0c 05 00 00                               	jbe    d6b49 <emuella_j2k_codestream::write_native_main_header+0x909>
   d663d:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6642:	66 42 c7 04 28 00 00                            	movw   $0x0,(%rax,%r13,1)
   d6649:	49 83 c5 02                                     	add    $0x2,%r13
   d664d:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6652:	49 8b 04 24                                     	mov    (%r12),%rax
   d6656:	4c 29 e8                                        	sub    %r13,%rax
   d6659:	48 83 f8 01                                     	cmp    $0x1,%rax
   d665d:	0f 86 0b 05 00 00                               	jbe    d6b6e <emuella_j2k_codestream::write_native_main_header+0x92e>
   d6663:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6668:	66 42 c7 04 28 00 01                            	movw   $0x100,(%rax,%r13,1)
   d666f:	49 83 c5 02                                     	add    $0x2,%r13
   d6673:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6678:	49 8b 04 24                                     	mov    (%r12),%rax
   d667c:	4c 29 e8                                        	sub    %r13,%rax
   d667f:	48 83 f8 05                                     	cmp    $0x5,%rax
   d6683:	0f 86 0a 05 00 00                               	jbe    d6b93 <emuella_j2k_codestream::write_native_main_header+0x953>
   d6689:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d668e:	0f b6 4c 24 06                                  	movzbl 0x6(%rsp),%ecx
   d6693:	42 88 0c 28                                     	mov    %cl,(%rax,%r13,1)
   d6697:	66 42 c7 44 28 01 02 04                         	movw   $0x402,0x1(%rax,%r13,1)
   d669f:	42 c6 44 28 03 04                               	movb   $0x4,0x3(%rax,%r13,1)
   d66a5:	0f b6 4c 24 07                                  	movzbl 0x7(%rsp),%ecx
   d66aa:	42 88 4c 28 04                                  	mov    %cl,0x4(%rax,%r13,1)
   d66af:	42 c6 44 28 05 01                               	movb   $0x1,0x5(%rax,%r13,1)
   d66b5:	49 83 c5 06                                     	add    $0x6,%r13
   d66b9:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d66be:	49 8b 04 24                                     	mov    (%r12),%rax
   d66c2:	4c 29 e8                                        	sub    %r13,%rax
   d66c5:	48 83 f8 01                                     	cmp    $0x1,%rax
   d66c9:	0f 86 e9 04 00 00                               	jbe    d6bb8 <emuella_j2k_codestream::write_native_main_header+0x978>
   d66cf:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d66d4:	66 42 c7 04 28 ff 5c                            	movw   $0x5cff,(%rax,%r13,1)
   d66db:	49 83 c5 02                                     	add    $0x2,%r13
   d66df:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d66e4:	49 8b 04 24                                     	mov    (%r12),%rax
   d66e8:	4c 29 e8                                        	sub    %r13,%rax
   d66eb:	48 83 f8 01                                     	cmp    $0x1,%rax
   d66ef:	0f 86 e8 04 00 00                               	jbe    d6bdd <emuella_j2k_codestream::write_native_main_header+0x99d>
   d66f5:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d66fa:	66 42 c7 04 28 00 0a                            	movw   $0xa00,(%rax,%r13,1)
   d6701:	49 8d 45 02                                     	lea    0x2(%r13),%rax
   d6705:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d670a:	49 3b 04 24                                     	cmp    (%r12),%rax
   d670e:	75 09                                           	jne    d6719 <emuella_j2k_codestream::write_native_main_header+0x4d9>
   d6710:	4c 89 e7                                        	mov    %r12,%rdi
   d6713:	ff 15 d7 76 19 00                               	call   *0x1976d7(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d6719:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d671e:	42 c6 44 28 02 40                               	movb   $0x40,0x2(%rax,%r13,1)
   d6724:	49 8d 45 03                                     	lea    0x3(%r13),%rax
   d6728:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d672d:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d6732:	0f b6 29                                        	movzbl (%rcx),%ebp
   d6735:	49 3b 04 24                                     	cmp    (%r12),%rax
   d6739:	75 09                                           	jne    d6744 <emuella_j2k_codestream::write_native_main_header+0x504>
   d673b:	4c 89 e7                                        	mov    %r12,%rdi
   d673e:	ff 15 ac 76 19 00                               	call   *0x1976ac(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d6744:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d6748:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d674d:	42 88 6c 28 03                                  	mov    %bpl,0x3(%rax,%r13,1)
   d6752:	49 8d 45 04                                     	lea    0x4(%r13),%rax
   d6756:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d675b:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d6760:	0f b6 69 01                                     	movzbl 0x1(%rcx),%ebp
   d6764:	49 3b 04 24                                     	cmp    (%r12),%rax
   d6768:	75 09                                           	jne    d6773 <emuella_j2k_codestream::write_native_main_header+0x533>
   d676a:	4c 89 e7                                        	mov    %r12,%rdi
   d676d:	ff 15 7d 76 19 00                               	call   *0x19767d(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d6773:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d6777:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d677c:	42 88 6c 28 04                                  	mov    %bpl,0x4(%rax,%r13,1)
   d6781:	49 8d 45 05                                     	lea    0x5(%r13),%rax
   d6785:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d678a:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d678f:	0f b6 69 02                                     	movzbl 0x2(%rcx),%ebp
   d6793:	49 3b 04 24                                     	cmp    (%r12),%rax
   d6797:	75 09                                           	jne    d67a2 <emuella_j2k_codestream::write_native_main_header+0x562>
   d6799:	4c 89 e7                                        	mov    %r12,%rdi
   d679c:	ff 15 4e 76 19 00                               	call   *0x19764e(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d67a2:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d67a6:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d67ab:	42 88 6c 28 05                                  	mov    %bpl,0x5(%rax,%r13,1)
   d67b0:	49 8d 45 06                                     	lea    0x6(%r13),%rax
   d67b4:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d67b9:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d67be:	0f b6 69 03                                     	movzbl 0x3(%rcx),%ebp
   d67c2:	49 3b 04 24                                     	cmp    (%r12),%rax
   d67c6:	75 09                                           	jne    d67d1 <emuella_j2k_codestream::write_native_main_header+0x591>
   d67c8:	4c 89 e7                                        	mov    %r12,%rdi
   d67cb:	ff 15 1f 76 19 00                               	call   *0x19761f(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d67d1:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d67d5:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d67da:	42 88 6c 28 06                                  	mov    %bpl,0x6(%rax,%r13,1)
   d67df:	49 8d 45 07                                     	lea    0x7(%r13),%rax
   d67e3:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d67e8:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d67ed:	0f b6 69 04                                     	movzbl 0x4(%rcx),%ebp
   d67f1:	49 3b 04 24                                     	cmp    (%r12),%rax
   d67f5:	75 09                                           	jne    d6800 <emuella_j2k_codestream::write_native_main_header+0x5c0>
   d67f7:	4c 89 e7                                        	mov    %r12,%rdi
   d67fa:	ff 15 f0 75 19 00                               	call   *0x1975f0(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d6800:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d6804:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6809:	42 88 6c 28 07                                  	mov    %bpl,0x7(%rax,%r13,1)
   d680e:	49 8d 45 08                                     	lea    0x8(%r13),%rax
   d6812:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d6817:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d681c:	0f b6 69 05                                     	movzbl 0x5(%rcx),%ebp
   d6820:	49 3b 04 24                                     	cmp    (%r12),%rax
   d6824:	75 09                                           	jne    d682f <emuella_j2k_codestream::write_native_main_header+0x5ef>
   d6826:	4c 89 e7                                        	mov    %r12,%rdi
   d6829:	ff 15 c1 75 19 00                               	call   *0x1975c1(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d682f:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d6833:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6838:	42 88 6c 28 08                                  	mov    %bpl,0x8(%rax,%r13,1)
   d683d:	49 8d 45 09                                     	lea    0x9(%r13),%rax
   d6841:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d6846:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d684b:	0f b6 69 06                                     	movzbl 0x6(%rcx),%ebp
   d684f:	49 3b 04 24                                     	cmp    (%r12),%rax
   d6853:	75 09                                           	jne    d685e <emuella_j2k_codestream::write_native_main_header+0x61e>
   d6855:	4c 89 e7                                        	mov    %r12,%rdi
   d6858:	ff 15 92 75 19 00                               	call   *0x197592(%rip)        # 26ddf0 <_DYNAMIC+0x270>
   d685e:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d6862:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d6867:	42 88 6c 28 09                                  	mov    %bpl,0x9(%rax,%r13,1)
   d686c:	49 83 c5 0a                                     	add    $0xa,%r13
   d6870:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d6875:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
   d687a:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
   d6881:	66 85 db                                        	test   %bx,%bx
   d6884:	75 2d                                           	jne    d68b3 <emuella_j2k_codestream::write_native_main_header+0x673>
   d6886:	e9 84 00 00 00                                  	jmp    d690f <emuella_j2k_codestream::write_native_main_header+0x6cf>
   d688b:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d6895:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d689a:	48 89 01                                        	mov    %rax,(%rcx)
   d689d:	c6 41 0f 00                                     	movb   $0x0,0xf(%rcx)
   d68a1:	66 c7 41 0d 00 00                               	movw   $0x0,0xd(%rcx)
   d68a7:	c7 41 09 00 00 00 00                            	movl   $0x0,0x9(%rcx)
   d68ae:	66 85 db                                        	test   %bx,%bx
   d68b1:	74 5c                                           	je     d690f <emuella_j2k_codestream::write_native_main_header+0x6cf>
   d68b3:	4c 89 f7                                        	mov    %r14,%rdi
   d68b6:	48 83 c4 28                                     	add    $0x28,%rsp
   d68ba:	5b                                              	pop    %rbx
   d68bb:	41 5c                                           	pop    %r12
   d68bd:	41 5d                                           	pop    %r13
   d68bf:	41 5e                                           	pop    %r14
   d68c1:	41 5f                                           	pop    %r15
   d68c3:	5d                                              	pop    %rbp
   d68c4:	ff 25 06 75 19 00                               	jmp    *0x197506(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   d68ca:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d68d4:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d68d9:	48 89 01                                        	mov    %rax,(%rcx)
   d68dc:	66 c7 41 0e 00 00                               	movw   $0x0,0xe(%rcx)
   d68e2:	c7 41 0a 00 00 00 00                            	movl   $0x0,0xa(%rcx)
   d68e9:	eb c8                                           	jmp    d68b3 <emuella_j2k_codestream::write_native_main_header+0x673>
   d68eb:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d68f5:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d68fa:	48 89 01                                        	mov    %rax,(%rcx)
   d68fd:	66 c7 41 0e 00 00                               	movw   $0x0,0xe(%rcx)
   d6903:	c7 41 0a 00 00 00 00                            	movl   $0x0,0xa(%rcx)
   d690a:	66 85 db                                        	test   %bx,%bx
   d690d:	75 a4                                           	jne    d68b3 <emuella_j2k_codestream::write_native_main_header+0x673>
   d690f:	48 83 c4 28                                     	add    $0x28,%rsp
   d6913:	5b                                              	pop    %rbx
   d6914:	41 5c                                           	pop    %r12
   d6916:	41 5d                                           	pop    %r13
   d6918:	41 5e                                           	pop    %r14
   d691a:	41 5f                                           	pop    %r15
   d691c:	5d                                              	pop    %rbp
   d691d:	c3                                              	ret
   d691e:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6923:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6928:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d692e:	4c 89 e7                                        	mov    %r12,%rdi
   d6931:	4c 89 ee                                        	mov    %r13,%rsi
   d6934:	e8 b7 3b fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6939:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d693e:	e9 24 fa ff ff                                  	jmp    d6367 <emuella_j2k_codestream::write_native_main_header+0x127>
   d6943:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6948:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d694d:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6953:	4c 89 e7                                        	mov    %r12,%rdi
   d6956:	4c 89 ee                                        	mov    %r13,%rsi
   d6959:	e8 92 3b fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d695e:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6963:	e9 25 fa ff ff                                  	jmp    d638d <emuella_j2k_codestream::write_native_main_header+0x14d>
   d6968:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d696d:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6972:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6978:	4c 89 e7                                        	mov    %r12,%rdi
   d697b:	4c 89 ee                                        	mov    %r13,%rsi
   d697e:	e8 6d 3b fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6983:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6988:	e9 26 fa ff ff                                  	jmp    d63b3 <emuella_j2k_codestream::write_native_main_header+0x173>
   d698d:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6992:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6997:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d699d:	4c 89 e7                                        	mov    %r12,%rdi
   d69a0:	4c 89 ee                                        	mov    %r13,%rsi
   d69a3:	e8 48 3b fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d69a8:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d69ad:	e9 2c fa ff ff                                  	jmp    d63de <emuella_j2k_codestream::write_native_main_header+0x19e>
   d69b2:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d69b7:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d69bc:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d69c2:	4c 89 e7                                        	mov    %r12,%rdi
   d69c5:	4c 89 ee                                        	mov    %r13,%rsi
   d69c8:	e8 23 3b fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d69cd:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d69d2:	e9 2d fa ff ff                                  	jmp    d6404 <emuella_j2k_codestream::write_native_main_header+0x1c4>
   d69d7:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d69dc:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d69e1:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d69e7:	4c 89 e7                                        	mov    %r12,%rdi
   d69ea:	4c 89 ee                                        	mov    %r13,%rsi
   d69ed:	e8 fe 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d69f2:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d69f7:	e9 31 fa ff ff                                  	jmp    d642d <emuella_j2k_codestream::write_native_main_header+0x1ed>
   d69fc:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d6a01:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6a06:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6a0c:	4c 89 e7                                        	mov    %r12,%rdi
   d6a0f:	4c 89 ee                                        	mov    %r13,%rsi
   d6a12:	e8 d9 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6a17:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6a1c:	e9 35 fa ff ff                                  	jmp    d6456 <emuella_j2k_codestream::write_native_main_header+0x216>
   d6a21:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d6a26:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6a2b:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6a31:	4c 89 e7                                        	mov    %r12,%rdi
   d6a34:	4c 89 ee                                        	mov    %r13,%rsi
   d6a37:	e8 b4 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6a3c:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6a41:	e9 37 fa ff ff                                  	jmp    d647d <emuella_j2k_codestream::write_native_main_header+0x23d>
   d6a46:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d6a4b:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6a50:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6a56:	4c 89 e7                                        	mov    %r12,%rdi
   d6a59:	4c 89 ee                                        	mov    %r13,%rsi
   d6a5c:	e8 8f 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6a61:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6a66:	e9 39 fa ff ff                                  	jmp    d64a4 <emuella_j2k_codestream::write_native_main_header+0x264>
   d6a6b:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d6a70:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6a75:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6a7b:	4c 89 e7                                        	mov    %r12,%rdi
   d6a7e:	4c 89 ee                                        	mov    %r13,%rsi
   d6a81:	e8 6a 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6a86:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6a8b:	e9 3d fa ff ff                                  	jmp    d64cd <emuella_j2k_codestream::write_native_main_header+0x28d>
   d6a90:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d6a95:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6a9a:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6aa0:	4c 89 e7                                        	mov    %r12,%rdi
   d6aa3:	4c 89 ee                                        	mov    %r13,%rsi
   d6aa6:	e8 45 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6aab:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6ab0:	e9 41 fa ff ff                                  	jmp    d64f6 <emuella_j2k_codestream::write_native_main_header+0x2b6>
   d6ab5:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d6aba:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6abf:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6ac5:	4c 89 e7                                        	mov    %r12,%rdi
   d6ac8:	4c 89 ee                                        	mov    %r13,%rsi
   d6acb:	e8 20 3a fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6ad0:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6ad5:	e9 43 fa ff ff                                  	jmp    d651d <emuella_j2k_codestream::write_native_main_header+0x2dd>
   d6ada:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6adf:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6ae4:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6aea:	4c 89 e7                                        	mov    %r12,%rdi
   d6aed:	4c 89 ee                                        	mov    %r13,%rsi
   d6af0:	e8 fb 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6af5:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6afa:	e9 45 fa ff ff                                  	jmp    d6544 <emuella_j2k_codestream::write_native_main_header+0x304>
   d6aff:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6b04:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6b09:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6b0f:	4c 89 e7                                        	mov    %r12,%rdi
   d6b12:	4c 89 ee                                        	mov    %r13,%rsi
   d6b15:	e8 d6 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6b1a:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6b1f:	e9 cd fa ff ff                                  	jmp    d65f1 <emuella_j2k_codestream::write_native_main_header+0x3b1>
   d6b24:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6b29:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6b2e:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6b34:	4c 89 e7                                        	mov    %r12,%rdi
   d6b37:	4c 89 ee                                        	mov    %r13,%rsi
   d6b3a:	e8 b1 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6b3f:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6b44:	e9 ce fa ff ff                                  	jmp    d6617 <emuella_j2k_codestream::write_native_main_header+0x3d7>
   d6b49:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6b4e:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6b53:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6b59:	4c 89 e7                                        	mov    %r12,%rdi
   d6b5c:	4c 89 ee                                        	mov    %r13,%rsi
   d6b5f:	e8 8c 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6b64:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6b69:	e9 cf fa ff ff                                  	jmp    d663d <emuella_j2k_codestream::write_native_main_header+0x3fd>
   d6b6e:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6b73:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6b78:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6b7e:	4c 89 e7                                        	mov    %r12,%rdi
   d6b81:	4c 89 ee                                        	mov    %r13,%rsi
   d6b84:	e8 67 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6b89:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6b8e:	e9 d0 fa ff ff                                  	jmp    d6663 <emuella_j2k_codestream::write_native_main_header+0x423>
   d6b93:	ba 06 00 00 00                                  	mov    $0x6,%edx
   d6b98:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6b9d:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6ba3:	4c 89 e7                                        	mov    %r12,%rdi
   d6ba6:	4c 89 ee                                        	mov    %r13,%rsi
   d6ba9:	e8 42 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6bae:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6bb3:	e9 d1 fa ff ff                                  	jmp    d6689 <emuella_j2k_codestream::write_native_main_header+0x449>
   d6bb8:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6bbd:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6bc2:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6bc8:	4c 89 e7                                        	mov    %r12,%rdi
   d6bcb:	4c 89 ee                                        	mov    %r13,%rsi
   d6bce:	e8 1d 39 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6bd3:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6bd8:	e9 f2 fa ff ff                                  	jmp    d66cf <emuella_j2k_codestream::write_native_main_header+0x48f>
   d6bdd:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d6be2:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d6be7:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d6bed:	4c 89 e7                                        	mov    %r12,%rdi
   d6bf0:	4c 89 ee                                        	mov    %r13,%rsi
   d6bf3:	e8 f8 38 fd ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d6bf8:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d6bfd:	e9 f3 fa ff ff                                  	jmp    d66f5 <emuella_j2k_codestream::write_native_main_header+0x4b5>
   d6c02:	bf 01 00 00 00                                  	mov    $0x1,%edi
   d6c07:	4c 89 ee                                        	mov    %r13,%rsi
   d6c0a:	ff 15 88 71 19 00                               	call   *0x197188(%rip)        # 26dd98 <_DYNAMIC+0x218>
   d6c10:	eb 00                                           	jmp    d6c12 <emuella_j2k_codestream::write_native_main_header+0x9d2>
   d6c12:	49 89 c7                                        	mov    %rax,%r15
   d6c15:	66 85 db                                        	test   %bx,%bx
   d6c18:	74 09                                           	je     d6c23 <emuella_j2k_codestream::write_native_main_header+0x9e3>
   d6c1a:	4c 89 f7                                        	mov    %r14,%rdi
   d6c1d:	ff 15 ad 71 19 00                               	call   *0x1971ad(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   d6c23:	4c 89 ff                                        	mov    %r15,%rdi
   d6c26:	e8 85 e8 18 00                                  	call   2654b0 <_Unwind_Resume@plt>
