import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
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
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { useDeleteProjectsMutation, useGetProjectsByIdQuery, useUpdateProjectsMutation } from '@/hooks'
import { Layout } from '@/layouts'
import { zodResolver } from '@hookform/resolvers/zod'
import { filter, get, includes, map } from 'lodash'
import { CopyIcon } from 'lucide-react'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { z } from 'zod'

const formSchema = z.object({
  name: z.string().min(2, {
    message: 'This field is required',
  }),
  options: z.object({
    hook: z.string().optional(),
    record: z.boolean().optional(),
  }),
  codecs: z.array(z.string()),
})

const options = [
  {
    id: 'record',
    label: 'Record',
    type: 'checkbox',
  },
  {
    id: 'hook',
    label: 'Hook',
    type: 'input',
  },
]

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
      <div className="max-w-xl">
        <div className="grid gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <Card className="shadow-sm">
                <CardHeader>
                  <CardTitle>General</CardTitle>
                  <CardDescription>Update your app name and other settings here.</CardDescription>
                </CardHeader>
                <CardContent className="grid gap-4">
                  <FormField
                    control={form.control}
                    name="name"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Project name</FormLabel>
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
                        <Input readOnly value={projectsById?.id} />
                        <Button type="button" variant="outline" size="icon">
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
                        <Input readOnly value={projectsById?.secret} />
                        <Button type="button" variant="outline" size="icon">
                          <CopyIcon />
                        </Button>
                      </div>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                  <FormField
                    control={form.control}
                    name="options"
                    render={() => (
                      <FormItem>
                        <FormLabel>Options</FormLabel>
                        {map(options, (o) => (
                          <FormField
                            key={o.id}
                            control={form.control}
                            name="options"
                            render={({ field }) => {
                              return (
                                <FormItem key={o.id} className="flex flex-row items-start space-x-3 space-y-0">
                                  <FormLabel className="font-normal">{o.label}</FormLabel>
                                  <FormControl>
                                    {o.type === 'checkbox' ? (
                                      <Checkbox
                                        checked={get(field.value, o.id)}
                                        onCheckedChange={(checked) => {
                                          return checked
                                            ? field.onChange({ ...field.value, [o.id]: true })
                                            : field.onChange({ ...field.value, [o.id]: false })
                                        }}
                                      />
                                    ) : (
                                      <Input
                                        value={get(field.value, o.id)}
                                        onChange={(e) => {
                                          console.log(e.target.value)
                                          field.onChange({ ...field.value, [o.id]: e.target.value })
                                        }}
                                      />
                                    )}
                                  </FormControl>
                                </FormItem>
                              )
                            }}
                          />
                        ))}
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="codecs"
                    render={() => (
                      <FormItem>
                        <FormLabel>Enabled codecs</FormLabel>
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
                  <Button variant="destructive">Delete this project</Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Are you absolutely sure?</DialogTitle>
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
                      I have read and understand these effects
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
