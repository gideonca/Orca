use std::{ptr, sync::Arc};

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    image::{view::ImageView, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    library::VulkanLibrary,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, Subpass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub(crate) fn create_window() {
    /*
     * This code snippet is setting up the Vulkan instance and required extensions for creating a
     * window using the `winit` library. Here's a breakdown of what each step does:
     */
    let event_loop = EventLoop::new();
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");

    // Enable all non core extensions required to draw a window
    let required_extensions = Surface::required_extensions(&event_loop);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create instance");

    /*
     * Create the actual window
     * This code snippet is creating a window using the `winit` library, creating a surface from that
     * window using Vulkan, and setting up device extensions required for Vulkan operations.
     */
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    /*
     * Setup the physical device (CPU or GPU)
     * This code snippet is responsible for finding a suitable physical device and queue family index
     * for rendering graphics using Vulkan. Here's a breakdown of what it does:
     */
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("No suitable physical device found");

    /*
     * The `println!` macro in the provided code snippet is used to print out the information about the
     * physical device being used for rendering graphics with Vulkan. It displays the name of the
     * device and its type (e.g., Discrete GPU, Integrated GPU, Virtual GPU, CPU, Other) to the console
     * for debugging and informational purposes. This helps in identifying the specific physical device
     * that the Vulkan application is utilizing for rendering operations.
     */
    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    /*
     * This code snippet is creating a new Vulkan device using the `Device::new` function. It takes the
     * `physical_device` (selected physical device for rendering) and a `DeviceCreateInfo` struct as
     * parameters.
     */
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .unwrap();

    /*
     * let queue = queues.next().unwrap();` is extracting the next available queue from the iterator
     * `queues` and unwrapping the result. In Vulkan, queues are used to submit command buffers for
     * execution on the GPU. By calling `queues.next()`, you are getting the next queue available for
     * use from the iterator. The `unwrap()` method is used here to immediately get the value of the
     * next queue or panic if there are no more queues available.
     */
    let queue = queues.next().unwrap();

    /*
     * This code snippet is responsible for creating a new swapchain for rendering images on the screen
     * using Vulkan. Here's a breakdown of what each step is doing:
     */
    let (mut swapchain, images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        /*
         * This code snippet is retrieving the supported surface formats for the given surface from the
         * physical device.
         */
        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        /*
         * This code snippet is creating a new swapchain for rendering images on the screen using
         * Vulkan. Here's a breakdown of what each parameter in `Swapchain::new` is doing:
         */
        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    /*
     * `let memory_alloator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));` is
     * creating a new instance of `StandardMemoryAllocator` using the `new_default` method with the
     * Vulkan device `device` cloned. The `Arc::new` is used to wrap the allocator in an `Arc` (atomic
     * reference counting) smart pointer, allowing multiple parts of the code to share ownership of the
     * allocator. This allocator is used for managing memory allocations in Vulkan, ensuring efficient
     * memory usage and allocation for various Vulkan resources like buffers and images.
     */
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    /* The above code defines a Rust struct named Vertex with a position field represented by a 2D
     * array of f32 values.
     *
     * Properties:
     * `position`: The `position` property in the `Vertex` struct represents the 2D position of a
     * vertex in space. It is defined as an array of two `f32` values, which typically represent the X
     * and Y coordinates of the vertex. The `#[format(R32G32_SFLOAT)]
     */
    #[derive(BufferContents, Vertex)]
    #[repr(C)]
    struct Vertex {
        #[format(R32G32_SFLOAT)]
        position: [f32; 2],
    }

    /*
     * This code snippet is defining an array named `vertices` that contains three instances of the
     * `Vertex` struct. Each `Vertex` instance represents a vertex in 2D space with a position defined
     * by an array of two `f32` values.
     */
    let vertices = [
        Vertex {
            position: [-0.5, -0.25],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.25, -0.1],
        },
    ];

    /*
     * This code snippet is creating a vertex buffer in Vulkan using the `Buffer::from_iter` method.
     * Here's a breakdown of what each part of the code is doing:
     */
    let vertex_buffer = Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vertices,
    )
    .unwrap();

    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: r"
                #version 450
                layout(location = 0) in vec2 position;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
        }
    }

    /// The `mod fs { ... }` block in the provided Rust code is defining a module named `fs` that
    /// contains a fragment shader. Here's a breakdown of what it's doing:
    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: r"
                #version 450

                layout(location = 0) out vec4 f_color;

                void main() {
                    f_color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            ",
        }
    }

    /*
     * The code snippet is written in Rust and it appears to be loading a vertex shader and a fragment
     * shader from a device. It then sets the entry point for both shaders to "main". However, there
     * seems to be a mistake in the code as it is loading the vertex shader twice instead of loading
     * the fragment shader
     */
    let vs = vs::load(device.clone())
        .unwrap()
        .entry_point("main")
        .unwrap();
    let fs = fs::load(device.clone())
        .unwrap()
        .entry_point("main")
        .unwrap();

    /* The above Rust code is creating a single-pass render pass using the `vulkano` crate. This render
     * pass defines a color attachment with the format of the swapchain's image format, 1 sample, and
     * clear and store operations. The render pass does not have a depth-stencil attachment.
     */
    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
    .unwrap();

    let vertex_input_state = Vertex::per_vertex()
        .definition(&vs.info().input_interface)
        .unwrap();

    let stages = [
        PipelineShaderStageCreateInfo::new(vs),
        PipelineShaderStageCreateInfo::new(fs),
    ]
    .into_iter()
    .collect();

    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let pipeline = GraphicsPipeline::new(
        device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages,
            vertex_input_state: Some(vertex_input_state),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState::default()),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::with_attachment_states(
                Subpass::from(render_pass.clone(), 0)
                    .unwrap()
                    .num_color_attachments(),
                ColorBlendAttachmentState::default(),
            )),
            dynamic_state: [DynamicState::Viewport].into_iter().collect(),
            subpass: Some(Subpass::from(render_pass.clone(), 0).unwrap().into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )
    .unwrap();

    let mut viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [0.0, 0.0],
        depth_range: 0.0..=1.0,
    };

    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    let mut framebuffers = images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    /*
     * command_buffer_allocator manages creation and destruction commands for drawing
     * recreate_swapchain indicates if swap_chain needs recreated
     * previous_frame_end obtains current GPU status
     */
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());
    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    /* Event Loop, this blocks the main thread forever
     * This code snippet is setting up an event loop that listens for window events. When a
     * `WindowEvent::CloseRequested` event is triggered, it sets the `control_flow` variable to
     * `ControlFlow::Exit`, which will cause the event loop to exit, effectively closing the window and
     * stopping the program.
     */
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            recreate_swapchain = true;
        }
        Event::RedrawEventsCleared => {
            let image_extent: [u32; 2] = window.inner_size().into();
            if image_extent.contains(&0) {
                return;
            }

            previous_frame_end.as_mut().unwrap().cleanup_finished();

            if recreate_swapchain {
                let (new_swapchain, new_images) = swapchain
                    .recreate(SwapchainCreateInfo {
                        image_extent,
                        ..swapchain.create_info()
                    })
                    .expect("failed to recreate swapchain");

                swapchain = new_swapchain;

                framebuffers = new_images
                    .iter()
                    .map(|image| {
                        let view = ImageView::new_default(image.clone()).unwrap();
                        Framebuffer::new(
                            render_pass.clone(),
                            FramebufferCreateInfo {
                                attachments: vec![view],
                                ..Default::default()
                            },
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();

                recreate_swapchain = false;
            }

            let (image_index, suboptimal, acquire_future) =
                match acquire_next_image(swapchain.clone(), None).map_err(Validated::unwrap) {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}"),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary(
                &command_buffer_allocator,
                queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                        ..RenderPassBeginInfo::framebuffer(
                            framebuffers[image_index as usize].clone(),
                        )
                    },
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                )
                .unwrap()
                .set_viewport(0, [viewport.clone()].into_iter().collect())
                .unwrap()
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .unwrap()
                .draw(vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap()
                .end_render_pass(Default::default())
                .unwrap();

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                )
                .then_signal_fence_and_flush();

            match future.map_err(Validated::unwrap) {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(VulkanError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    panic!("failed to flush future: {e}");
                }
            }
        }
        _ => (),
    });
}
