import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Checkbox } from '@/components/ui/checkbox'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { useDeleteProjectsMutation, useGetProjectsByIdQuery, useToast, useUpdateProjectsMutation } from '@/hooks'
import { Layout } from '@/layouts'
import { zodResolver } from '@hookform/resolvers/zod'
import { filter, get, includes, map } from 'lodash'
import { CopyIcon, EyeIcon, EyeOffIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import { useForm } from 'react-hook-form'
import { useCopyToClipboard } from 'usehooks-ts'
import { z } from 'zod'

const formSchema = z.object({
  name: z.string().min(2, {
    message: 'This field is required.',
  }),
  options: z.object({
    record: z.boolean().optional(),
    hook: z.string().optional(),
  }),
  codecs: z.array(z.string()),
})

const codecs = [
  {
    id: 'h264',
    label: 'h264',
  },
  {
    id: 'opus',
    label: 'opus',
  },
  {
    id: 'vp8',
    label: 'vp8',
  },
  {
    id: 'vp9',
    label: 'vp9',
  },
]

export const ProjectsSettings = () => {
  const { toast } = useToast()
  const [, copy] = useCopyToClipboard()
  const { data: projectsById } = useGetProjectsByIdQuery()
  const { mutate: onUpdateProjects, isPending: isPendingUpdateProjects } = useUpdateProjectsMutation()
  const { mutate: onDeleteProjects, isPending: isPendingDeleteProjects } = useDeleteProjectsMutation()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: projectsById?.name,
      options: {
        hook: projectsById?.options?.hook,
        record: projectsById?.options?.record,
      },
      codecs: [],
    },
  })
  const [visibleProjectSecret, setVisibleProjectSecret] = useState(false)

  useEffect(() => {
    if (projectsById) {
      form.setValue('name', projectsById?.name)

      const options = {
        hook: projectsById?.options?.hook,
        record: projectsById?.options?.record,
      }
      form.setValue('options', options)

      const codecs = []
      if (projectsById?.codecs?.h264) {
        codecs.push('h264')
      }
      if (projectsById?.codecs?.opus) {
        codecs.push('opus')
      }
      if (projectsById?.codecs?.vp8) {
        codecs.push('vp8')
      }
      if (projectsById?.codecs?.vp9) {
        codecs.push('vp9')
      }
      form.setValue('codecs', codecs)
    }
  }, [form, projectsById, projectsById?.name])

  const onSubmit = (values: z.infer<typeof formSchema>) => {
    onUpdateProjects({
      id: projectsById?.id as string,
      data: {
        name: values.name,
        options: {
          hook: get(values.options, 'hook'),
          record: get(values.options, 'record') ?? false,
        },
        codecs: {
          h264: includes(values.codecs, 'h264'),
          opus: includes(values.codecs, 'opus'),
          vp8: includes(values.codecs, 'vp8'),
          vp9: includes(values.codecs, 'vp9'),
        },
      },
    })
  }
  return (
    <Layout>
      <div className="max-w-3xl">
        <div className="grid gap-4">
          <div>
            <p className="mb-1 text-xl font-semibold">Settings</p>
            <p className="text-sm text-muted-foreground">
              Update your projects name and other settings here. Be careful with the settings as they can affect your
              project.
            </p>
          </div>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <Card className="shadow-sm">
                <CardContent className="grid gap-4 pt-6">
                  <FormField
                    control={form.control}
                    name="name"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Project Name</FormLabel>
                        <FormControl>
                          <Input placeholder="Enter your project name" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormItem>
                    <FormLabel>Project Id</FormLabel>
                    <FormControl>
                      <div className="flex items-center gap-2">
                        <Input readOnly value={projectsById?.id} className="flex-1" />
                        <Button
                          onClick={() => {
                            copy(projectsById?.id).then(() => {
                              toast({
                                title: 'Copied to clipboard',
                                description: 'Your project id has been copied to the clipboard.',
                              })
                            })
                          }}
                          type="button"
                          variant="outline"
                          size="icon"
                        >
                          <CopyIcon />
                        </Button>
                      </div>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                  <FormItem>
                    <FormLabel>Project Secret</FormLabel>
                    <FormControl>
                      <div className="flex items-center gap-2">
                        <Input
                          type={visibleProjectSecret ? 'text' : 'password'}
                          readOnly
                          value={projectsById?.secret}
                          className="flex-1"
                        />
                        <Button
                          onClick={() => {
                            setVisibleProjectSecret(!visibleProjectSecret)
                          }}
                          type="button"
                          variant="outline"
                          size="icon"
                        >
                          {!visibleProjectSecret ? <EyeIcon /> : <EyeOffIcon />}
                        </Button>
                        <Button
                          onClick={() => {
                            copy(projectsById?.secret).then(() => {
                              toast({
                                title: 'Copied to clipboard',
                                description: 'Your project secret has been copied to the clipboard.',
                              })
                            })
                          }}
                          type="button"
                          variant="outline"
                          size="icon"
                        >
                          <CopyIcon />
                        </Button>
                      </div>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                  <div className="grid gap-2">
                    <FormField
                      control={form.control}
                      name="options"
                      render={() => (
                        <FormItem>
                          <FormLabel>Options</FormLabel>
                          <FormField
                            control={form.control}
                            name="options"
                            render={({ field }) => {
                              return (
                                <>
                                  <FormItem className="flex flex-row items-center justify-between rounded-lg border p-3 shadow-sm">
                                    <div className="space-y-0.5">
                                      <FormLabel>Record</FormLabel>
                                      <FormDescription>
                                        Auto or manual recording of the meeting. This will be saved in the cloud.
                                      </FormDescription>
                                    </div>
                                    <FormControl>
                                      <Switch
                                        checked={field.value.record}
                                        onCheckedChange={(checked) => {
                                          field.onChange({ ...field.value, record: checked })
                                        }}
                                      />
                                    </FormControl>
                                  </FormItem>
                                  <FormItem className="flex flex-col rounded-lg border p-3 shadow-sm">
                                    <div className="space-y-0.5">
                                      <FormLabel>Hook</FormLabel>
                                      <FormDescription>
                                        Webhook URL to send events to. This can be used to send events to your server.
                                      </FormDescription>
                                    </div>
                                    <FormControl>
                                      <Input
                                        value={field.value.hook}
                                        onChange={(e) => {
                                          field.onChange({ ...field.value, hook: e.target.value })
                                        }}
                                      />
                                    </FormControl>
                                  </FormItem>
                                </>
                              )
                            }}
                          />
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="codecs"
                      render={() => (
                        <FormItem className="flex flex-col rounded-lg border p-3 shadow-sm">
                          <div className="space-y-0.5">
                            <FormLabel>Enabled codecs</FormLabel>
                            <FormDescription>Choose which codecs you want to enable for your project.</FormDescription>
                          </div>
                          {map(codecs, (c) => (
                            <FormField
                              key={c.id}
                              control={form.control}
                              name="codecs"
                              render={({ field }) => {
                                return (
                                  <FormItem key={c.id} className="flex flex-row items-start space-x-3 space-y-0">
                                    <FormControl>
                                      <Checkbox
                                        checked={includes(field.value, c.id)}
                                        onCheckedChange={(checked) => {
                                          return checked
                                            ? field.onChange([...field.value, c.id])
                                            : field.onChange(filter(field.value, (value) => value !== c.id))
                                        }}
                                      />
                                    </FormControl>
                                    <FormLabel className="font-normal">{c.label}</FormLabel>
                                  </FormItem>
                                )
                              }}
                            />
                          ))}
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>
                </CardContent>
                <CardFooter>
                  <Button loading={isPendingUpdateProjects} type="submit">
                    Save
                  </Button>
                </CardFooter>
              </Card>
            </form>
          </Form>
          <Card className="border-red-500 bg-red-500 bg-opacity-5 shadow-sm">
            <CardHeader>
              <CardTitle className="text-red-500">Danger zone</CardTitle>
            </CardHeader>
            <CardContent>
              <div>
                <p className="font-medium">Delete project</p>
                <p className="text-xs text-muted-foreground">
                  Once you delete a project, there is no going back. Please be certain.
                </p>
              </div>
            </CardContent>
            <CardFooter>
              <Dialog>
                <DialogTrigger>
                  <Button variant="destructive">Delete This Project</Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Are You Absolutely Sure?</DialogTitle>
                    <DialogDescription>
                      This action cannot be undone. This will permanently delete your project and remove your data from our
                      servers.
                    </DialogDescription>
                  </DialogHeader>
                  <DialogFooter>
                    <Button
                      loading={isPendingDeleteProjects}
                      onClick={() => {
                        onDeleteProjects({
                          id: projectsById?.id as string,
                        })
                      }}
                      variant="destructive"
                    >
                      I Have Read And Understand These Effects
                    </Button>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </CardFooter>
          </Card>
        </div>
      </div>
    </Layout>
  )
}
